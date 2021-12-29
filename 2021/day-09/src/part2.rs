use std::collections::HashSet;

use super::{Map, Point};
use super::part1::is_low_point;

#[derive(Debug)]
enum Edge {
    Top,
    Right,
    Bottom,
    Left,

    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

#[derive(Debug)]
enum Offset {
    Up,
    Down,
    Left,
    Right,
}

impl Offset {
    fn val(&self) -> (i8, i8) {
        match self {       // (+x, +y)
            Offset::Up =>     ( 0, -1),
            Offset::Down =>   ( 0,  1),
            Offset::Left =>   (-1,  0),
            Offset::Right =>  ( 1,  0),
        }
    }
}


fn check_edge(map: &Map, point: Point) -> Option<Edge> {
    let row_max = map.len() - 1;
    let col_max = map[0].len() - 1;

    let top = point.1 == 0;
    let right = point.0 == col_max;
    let bottom = point.1 == row_max;
    let left = point.0 == 0;

    if top && right { Some(Edge::TopRight) }
    else if top && left { Some(Edge::TopLeft) }
    else if bottom && right { Some(Edge::BottomRight) }
    else if bottom && left { Some(Edge::BottomLeft) }
    else if top { Some(Edge::Top) }
    else if right { Some(Edge::Right) }
    else if bottom { Some(Edge::Bottom) }
    else if left { Some(Edge::Left) }
    else { None }
}


fn checked_add_tuple(a: (usize, usize), b: Offset) -> (usize, usize) {
    fn add(a: usize, b: i8) -> usize {
        if b < 0 {
            a.saturating_sub(b.abs() as usize)
        } else {
            a.saturating_add(b.abs() as usize)
        }
    }

    let b = b.val();
    (add(a.0, b.0), add(a.1, b.1))
}


fn scan(map: &Map, visited: &mut HashSet<Point>, point: Point) {

    #[cfg(test)]
    print!("Checking point {:?}", point);

    // Check that, if this point has not been visited yet, it is not a 9
    if !visited.contains(&point) && map[point.1][point.0] != 9 {
        visited.insert(point);

        #[cfg(test)]
        print!(" -- was not in set");

        // Now check all the neighbours
        let offsets = if let Some(edge) = check_edge(map, point) {

            #[cfg(test)]
            println!(" -- on {:?} edge", edge);

            // If we're on an edge, we exclude the offset that would push us off the map
            match edge {
                Edge::Top => vec![ Offset::Left, Offset::Down, Offset::Right ],
                Edge::Right => vec![ Offset::Up, Offset::Left, Offset::Down ],
                Edge::Bottom => vec![ Offset::Left, Offset::Up, Offset::Right ],
                Edge::Left => vec![ Offset::Up, Offset::Right, Offset::Down ],

                Edge::TopRight => vec![ Offset::Left, Offset::Down ],
                Edge::TopLeft => vec![ Offset::Right, Offset::Down ],
                Edge::BottomRight => vec![ Offset::Left, Offset::Up ],
                Edge::BottomLeft => vec![ Offset::Right, Offset::Up ],
            }
        } else {

            #[cfg(test)]
            println!();

            vec![ Offset::Up, Offset::Right, Offset::Down, Offset::Left ]
        };

        for offset in offsets {
            let new_point = checked_add_tuple(point, offset);
            scan(map, visited, new_point);
        }
    } else {

        #[cfg(test)]
        println!();

    }
}


fn find_basin_size(map: &Map, point: Point) -> usize {
    let mut visited = HashSet::new();

    scan(map, &mut visited, point);

    #[cfg(test)]
    println!("Found basin size: {}\n", visited.len());

    visited.len()
}


pub fn run(map: &Map) -> usize {

    // Begin by finding all low-points

    let low_points = {
        let mut points = Vec::new();

        for y in 0..map.len() {
            for x in 0..map[0].len() {
                let p = (x, y);

                if is_low_point(map, p) {
                    points.push(p);
                }
            }
        }

        points
    };

    // Now for each of the low points we find the size of its basin
    let mut basin_sizes = low_points
        .iter()
        .map(|&point| find_basin_size(map, point))
        .collect::<Vec<_>>();

    #[cfg(test)]
    println!("All basin sizes: {:#?}", basin_sizes);

    // Sort in descending order to get the top three
    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes.truncate(3);

    basin_sizes.iter().fold(1, |acc, &cur| acc * cur)
}


#[cfg(test)]
mod tests {

    use super::run;
    use super::super::tests::example_data;

    #[test]
    fn example() {
        let map = example_data();
        assert_eq!(run(&map), 1134);
    }
}