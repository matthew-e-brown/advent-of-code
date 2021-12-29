type ParseResult<T> = Result<T, &'static str>;

pub type Map = Vec<Vec<u8>>;

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


// fn check_point(data: &Map, val: u8, p: (usize, usize)) -> bool {
//     // row -> column, so y (p.1) is before x (p.0)
//     match data.get(p.1) {
//         Some(row) => match row.get(p.0) {
//             Some(&n) => n > val,
//             None => true,
//         },
//         None => true,
//     }
// }


fn is_low_point(data: &Map, point: (usize, usize)) -> bool {

    let p = data[point.1][point.0];

    // Check that we are actually able to subtract from this point's x/y before doing so

    let row_max = data.len() - 1;
    let col_max = data[0].len() - 1;

    let c1 = || if point.0 > 0 { p < data[point.1][point.0 - 1] } else { true };
    let c2 = || if point.0 < col_max { p < data[point.1][point.0 + 1] } else { true };

    let c3 = || if point.1 > 0 { p < data[point.1 - 1][point.0] } else { true };
    let c4 = || if point.1 < row_max { p < data[point.1 + 1][point.0] } else { true };

    c1() && c2() && c3() && c4()
}


pub fn run_1(data: &Map) -> usize {

    let mut risk_level = 0;

    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if is_low_point(data, (x, y)) {
                risk_level += (data[y][x] as usize) + 1;
            }
        }
    }

    risk_level
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let map = parse(&vec![
            "2199943210".to_owned(),
            "3987894921".to_owned(),
            "9856789892".to_owned(),
            "8767896789".to_owned(),
            "9899965678".to_owned(),
        ]).unwrap();

        assert_eq!(run_1(&map), 15)
    }

}