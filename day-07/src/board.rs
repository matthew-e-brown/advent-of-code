use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::{Regex, Captures};

enum WireValue<'a> {
    Computed(u16),
    NoValue(Captures<'a>)
}

pub struct CircuitBoard<'a> {
    map: HashMap<String, WireValue<'a>>,
}


impl<'a> CircuitBoard<'a> {


    pub fn new(data: &'a Vec<String>) -> Result<Self, String> {

        // Maybe I like regular expressions a bit too much...
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)^(?:
                    (?P<LHS>[a-z]+|\d+)\ (?P<OP>AND|OR|RSHIFT|LSHIFT)\ (?P<RHS>[a-z]+|\d+)|NOT\ (?P<NOT_VAL>[a-z]+|\d+)
                    |(?P<VAL>[a-z]+|\d+)
                )\ ->\ (?P<RES>[a-z]+)$"
            ).unwrap();
        }

        let mut map = HashMap::new();

        for string in data.iter() {
            let caps = RE
                .captures(&string)
                .ok_or(format!("Encountered malformed line: `{}`", string))?;

            // Because of the strictness/anchors of the regex, we know that we must have one of the valid possible
            // forms, and we store it directly in the map, only to be parsed should we need it later.

            let k = caps.name("RES").unwrap().as_str().to_owned();

            if map.contains_key(&k) {
                return Err(format!("Encountered wire `{}` with more than one definition", k));
            }

            map.insert(k, WireValue::NoValue(caps));
        }

        Ok(Self { map })
    }


    fn unary_parse(
        &mut self,
        string: &str,
        action: fn(u16) -> u16,
    ) -> Result<u16, String> {

        string
            .parse()
            .and_then(|n| Ok(action(n)))
            .or_else(|_| {
                // If we couldn't get a value directly, that means we have a Wire instead of a raw number (or an invalid
                // value, which will error out). So we simply get the value of that wire:
                let n = self.get(string)?;
                Ok(action(n))
            })

    }


    fn binary_parse(
        &mut self,
        term_l: &str,
        term_r: &str,
        action: fn(u16, u16) -> u16,
    ) -> Result<u16, String> {

        let lhs = self.unary_parse(term_l, |n| n)?;
        let rhs = self.unary_parse(term_r, |n| n)?;
        Ok(action(lhs, rhs))

    }


    pub fn get(&mut self, key: &str) -> Result<u16, String> {

        match self.map.get(key) {
            None => Err(format!("Wire `{}` does not exist in this circuit board", key)),
            Some(wire_value) => {
                match wire_value {
                    WireValue::Computed(n) => Ok(*n),
                    WireValue::NoValue(caps) => {
                        // We can check which capture groups exist on that captures object, which will tell us what
                        // form the line is in.

                        // If wire_value is just a direct `wire -> wire` or `value -> wire`, it will have a VAL
                        // group
                        let new_val = if let Some(val) = caps.name("VAL") {
                            self.unary_parse(val.as_str(), |n| n)
                        }

                        // NOT_VAL group only exists if it's a `NOT wire` or `NOT value`
                        else if let Some(not_val) = caps.name("NOT_VAL") {
                            self.unary_parse(not_val.as_str(), |n| !n)
                        }

                        // Otherwise, the strictness of the regular expression tells us that any other line that
                        // made it this far is of the more generic `[wire|value] OPERATOR [wire|value]` form
                        else {
                            let term_l = caps.name("LHS").unwrap().as_str();
                            let term_r = caps.name("RHS").unwrap().as_str();
                            let operator = caps.name("OP").unwrap().as_str();

                            self.binary_parse(term_l, term_r, match operator {
                                "RSHIFT" =>   |n: u16, m: u16| n >> m,
                                "LSHIFT" =>   |n, m| n << m,
                                "AND" =>      |n, m| n & m,
                                "OR" =>       |n, m| n | m,
                                _ => unreachable!(),
                            })
                        }?;

                        // Now that we've computed it, we override the original string with the computed value so we
                        // don't have to do it again.
                        *(self.map.get_mut(key).unwrap()) = WireValue::Computed(new_val);
                        Ok(new_val)
                    }
                }
            } // end Some
        } // end match

    }

}