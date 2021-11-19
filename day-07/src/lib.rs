pub mod board;

use board::CircuitBoard;


pub fn run_1(board: &Vec<String>) -> Result<u16, String> {
    let mut board = CircuitBoard::new(board)?;
    board.get("a")
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
        let mut board = CircuitBoard::new(&lines).unwrap();

        assert_eq!(board.get(compute).unwrap(), expected);
    }

}