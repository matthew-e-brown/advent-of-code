use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

pub mod board;

type CircuitBoard<'a> = HashMap<&'a str, Captures<'a>>;


fn construct_board(strings: &Vec<String>) -> Result<CircuitBoard, String> {

    lazy_static! {
        // Maybe I like regular expressions a bit too much...
        static ref RE: Regex = Regex::new(
            r"(?x)
            ^
                (?:
                    (?P<LHS>[a-z]+|\d+)\ (?P<OP>AND|OR|RSHIFT|LSHIFT)\ (?P<RHS>[a-z]+|\d+)   # (wire|value) OPERATOR (wire|value) or
                    |NOT\ (?P<NOT_VAL>[a-z]+|\d+)                                            # NOT (wire|value) or
                    |(?P<VAL>[a-z]+|\d+)                                                     # (wire|value)
                )
                \ ->                                                                         # ->
                \ (?P<RES>[a-z]+)                                                            # wire
            $"
        ).unwrap();
    }

    let mut board = HashMap::new();

    for string in strings {
        let caps = RE
            .captures(string)
            .ok_or(format!("Encountered malformed line: `{}`", string))?;

        // Because of the strictness/anchors of the regex, we know that we must have one of the valid possible
        // forms, and we store it directly in the map, only to be parsed should we need it later.

        let k = caps.name("RES").unwrap().as_str();

        if board.contains_key(&k) {
            return Err(format!("Encountered wire `{}` with more than one definition", k));
        }

        board.insert(k, caps);
    }

    Ok(board)
}


fn parse_board(board: &CircuitBoard, compute: &str) -> Result<u16, String> {

    let unary_parse = |string: &str, action: fn(u16) -> u16| -> Result<u16, String> {
        println!("Looking for value to: {}", string);
        string
            .parse()
            .and_then(|n| Ok(action(n)))
            .or_else(|_| {
                // If we couldn't get a value directly, that means we have a Wire instead of a raw number (or an
                // invalid value, which will error out).
                let new_n = parse_board(board, string)?;
                Ok(action(new_n))
            })
    };

    let binary_parse = |lhs: &str, rhs: &str, action: fn(u16, u16) -> u16| -> Result<u16, String> {
        let lhs = unary_parse(lhs, |n| n)?;
        let rhs = unary_parse(rhs, |n| n)?;
        Ok(action(lhs, rhs))
    };


    // Look for the formula that makes up the line we're currently searching for.  
    //
    // i.e. if we had:
    //
    //      `lx RSHIFT 18 -> x`
    //
    // and we wanted `x`, this line would give us `"lx RSHIFT 18"` complete with capture group information.

    let line = board.get(compute).ok_or(format!("Found wire `{}` with no expression", compute))?;


    // We can then check which capture groups exist on that captures object, which will tell us what form the line
    // is in.


    // If the line is just a direct `wire -> wire` or `value -> wire`, it will have a VAL group
    if let Some(val) = line.name("VAL") {
        unary_parse(val.as_str(), |n| n)
    }

    // NOT_VAL group only exists if it's a `NOT wire` or `NOT value`
    else if let Some(operand) = line.name("NOT_VAL") {
        unary_parse(operand.as_str(), |n| !n)
    }

    // Otherwise, the strictness of the regular expression tells us that any other line that made it this far is of
    // the more generic `[wire|value] OPERATOR [wire|value]` form
    else {
        let term_l = line.name("LHS").unwrap().as_str();
        let term_r = line.name("RHS").unwrap().as_str();
        let operator = line.name("OP").unwrap().as_str();

        binary_parse(term_l, term_r, match operator {
            "RSHIFT" =>   |n: u16, m: u16| n >> m,
            "LSHIFT" =>   |n, m| n << m,
            "AND" =>      |n, m| n & m,
            "OR" =>       |n, m| n | m,
            _ => unreachable!(),
        })
    }

}


pub fn run_1(board: &Vec<String>) -> Result<u16, String> {
    let board = construct_board(&board)?;
    parse_board(&board, "a")
}


#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    fn test_lines() -> Vec<String> {
        vec![
            "123 -> x".to_owned(),
            "456 -> y".to_owned(),
            "x AND y -> d".to_owned(),
            "x OR y -> e".to_owned(),
            "x LSHIFT 2 -> f".to_owned(),
            "y RSHIFT 2 -> g".to_owned(),
            "NOT x -> h".to_owned(),
            "NOT y -> i".to_owned(),
        ]
    }


    #[test_case("d",     72 ; "case 1")]
    #[test_case("e",    507 ; "case 2")]
    #[test_case("f",    492 ; "case 3")]
    #[test_case("g",    114 ; "case 4")]
    #[test_case("h",  65412 ; "case 5")]
    #[test_case("i",  65079 ; "case 6")]
    #[test_case("x",    123 ; "case 7")]
    #[test_case("y",    456 ; "case 8")]
    fn example(compute: &str, expected: u16) {
        let lines = test_lines();
        let board = construct_board(&lines).unwrap();
        assert_eq!(parse_board(&board, compute).unwrap(), expected);
    }

}