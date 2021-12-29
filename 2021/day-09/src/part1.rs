use super::{Map, Point};

pub fn is_low_point(map: &Map, point: Point) -> bool {

    let p = map[point.1][point.0];

    // Check that we are actually able to subtract from this point's x/y before doing so

    let row_max = map.len() - 1;
    let col_max = map[0].len() - 1;

    let c1 = || if point.0 > 0 { p < map[point.1][point.0 - 1] } else { true };
    let c2 = || if point.0 < col_max { p < map[point.1][point.0 + 1] } else { true };

    let c3 = || if point.1 > 0 { p < map[point.1 - 1][point.0] } else { true };
    let c4 = || if point.1 < row_max { p < map[point.1 + 1][point.0] } else { true };

    c1() && c2() && c3() && c4()
}


pub fn run(data: &Map) -> usize {

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

    use super::run;
    use super::super::tests::example_data;

    #[test]
    fn example() {
        let map = example_data();
        assert_eq!(run(&map), 15);
    }
}