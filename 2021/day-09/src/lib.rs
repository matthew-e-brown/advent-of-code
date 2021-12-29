mod part1;
mod part2;

pub use part1::run as run_1;
pub use part2::run as run_2;

pub type Map = Vec<Vec<u8>>;
pub type Point = (usize, usize);

type ParseResult<T> = Result<T, &'static str>;

pub fn parse(data: &Vec<String>) -> ParseResult<Map> {
    let mut output = Vec::new();

    for line in data {
        let mut row = Vec::new();

        for char in line.chars() {
            let n = char.to_digit(10).ok_or("Found non-number character")?;
            if n > 9 {
                return Err("Found number > 9");
            }

            row.push(n as u8);
        }

        output.push(row);
    }

    Ok(output)
}


#[cfg(test)]
mod tests {

    use super::*;

    pub fn example_data() -> Map {
        let data = vec![
            "2199943210".to_owned(),
            "3987894921".to_owned(),
            "9856789892".to_owned(),
            "8767896789".to_owned(),
            "9899965678".to_owned(),
        ];

        let result = parse(&data);

        assert!(result.is_ok());

        result.unwrap()
    }
}