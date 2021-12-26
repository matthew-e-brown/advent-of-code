use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::{Regex, Captures};


enum WireValue {
    Computed(u16),
    NoValue,
}

struct Wire<'a> {
    src: Captures<'a>,
    val: WireValue,
}

pub struct CircuitBoard<'a> {
    map: HashMap<String, Wire<'a>>,
}


impl<'a> CircuitBoard<'a> {


    fn parse_string(string: &'a str) -> Result<Captures<'a>, String> {
        // Maybe I like regular expressions a bit too much...
        lazy_static! {
            static ref THE_BIG_KAHUNA: Regex = Regex::new(
                r"(?x)^(?:
                    (?P<LHS>[a-z]+|\d+)\ (?P<OP>AND|OR|RSHIFT|LSHIFT)\ (?P<RHS>[a-z]+|\d+)|NOT\ (?P<NOT_VAL>[a-z]+|\d+)
                    |(?P<VAL>[a-z]+|\d+)
                )\ ->\ (?P<RES>[a-z]+)$"
            ).unwrap();
        }

        THE_BIG_KAHUNA
            .captures(&string)
            .ok_or(format!("Encountered malformed line: `{}`", string))
    }


    pub fn new(data: &'a Vec<String>) -> Result<Self, String> {

        let mut map = HashMap::new();

        for string in data.iter() {
            let caps = Self::parse_string(string)?;

            // Because of the strictness/anchors of the regex, we know that we must have one of the valid possible
            // forms, and we store it directly in the map, only to be parsed should we need it later.

            let k = caps.name("RES").unwrap().as_str().to_owned();

            if map.contains_key(&k) {
                return Err(format!("Encountered wire `{}` with more than one definition", k));
            }

            map.insert(k, Wire { src: caps, val: WireValue::NoValue });
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


    pub fn get(&mut self, wire_tag: &str) -> Result<u16, String> {

        match self.map.get(wire_tag) {
            None => Err(format!("Wire `{}` does not exist on this circuit board", wire_tag)),
            Some(wire_value) => {
                match wire_value.val {
                    WireValue::Computed(n) => Ok(n),
                    WireValue::NoValue => {
                        // We can check which capture groups exist on that captures object, which will tell us what
                        // form the line is in.

                        let caps = &wire_value.src;

                        // If wire_value is just a direct `wire -> wire` or `value -> wire`, it will have a VAL
                        // group
                        let val = if let Some(val) = caps.name("VAL") {
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
                        self.map.get_mut(wire_tag).unwrap().val = WireValue::Computed(val);
                        Ok(val)
                    }
                }
            } // end Some
        } // end match

    }



    fn invalidate_wires(&mut self) {

        // Force all wires to be re-computed
        for wire in self.map.values_mut() {
            wire.val = WireValue::NoValue;
        }

    }



    pub fn add_wire(&mut self, string: &'a str) -> Result<(), String> {

        let caps = Self::parse_string(string)?;
        let k = caps.name("RES").unwrap().as_str().to_owned();

        if self.map.contains_key(&k) {
            Err(format!("Wire `{}` already has a definition", k))
        } else {
            self.map.insert(k, Wire { src: caps, val: WireValue::NoValue });
            Ok(())
        }

    }


    pub fn remove_wire(&mut self, wire_tag: &str) -> Result<(), String> {

        if self.map.contains_key(wire_tag) {
            self.map.remove(wire_tag);
            self.invalidate_wires();
            Ok(())
        } else {
            Err(format!("Wire `{}` not found on circuit board", wire_tag))
        }

    }

}