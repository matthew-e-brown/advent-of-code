use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;


#[derive(Debug, PartialEq, Eq, Hash)]
struct Route<'a> {
    source: &'a str,
    target: &'a str,
}


fn parse_data(data: &Vec<String>) -> Result<(HashMap<Route, usize>, HashSet<&str>), String> {

    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\w+)\s+to\s+(\w+)\s+=\s+([\d\.]+)$").unwrap();
    }

    let mut routes = HashMap::new();
    let mut cities = HashSet::new();

    for line in data.iter() {
        let caps = RE.captures(line).ok_or(format!("Malformed line: `{}`", line))?;

        // Using a regular expression to check validity of data allows us to unwrap on all operations below; if the data
        // were invalid, the function exit at .ok_or above

        let a = caps.get(1).unwrap().as_str();
        let b = caps.get(2).unwrap().as_str();
        let cost = caps.get(3).unwrap().as_str().parse().unwrap();

        cities.insert(a);
        cities.insert(b);

        let route = Route { source: a, target: b };
        routes.insert(route, cost);
    }


    Ok( (routes, cities) )
}


pub fn run_1(data: &Vec<String>) -> Result<(usize, String), String> {

    let (routes, cities) = parse_data(data)?;
    let permutations = cities.iter().permutations(cities.len());

    let mut min_dist = std::usize::MAX;
    let mut min_path = String::new();

    'o: for path in permutations {

        #[cfg(test)]
        print!("Permutation: {} = ", path.iter().join(" -> "));

        // The total distance of *this* current A->B->C path
        let mut dist = 0;

        // Loop through all the pairs of cities
        for walk in path.windows(2) {
            let a = walk[0];
            let b = walk[1];

            // Check that there actually exists a path between these two cities
            let walk_dist = {
                match routes.get(&Route { source: a, target: b }) {
                    Some(v) => Some(v),
                    // If not A->B, check B->A
                    None => match routes.get(&Route { source: b, target: a }) {
                        Some(v) => Some(v),
                        None => None,
                    },
                }
            };

            if let Some(d) = walk_dist {
                min_path = path.iter().join(" -> ");
                dist += d;
            } else {
                // This permutation of walks (this "path") does not have a way to go all the way from A->Z, so we ignore
                // it and try the next one
                continue 'o;
            }
        }

        #[cfg(test)]
        println!("{}", dist);

        if dist < min_dist { min_dist = dist; }
    }

    Ok((min_dist, min_path))
}


#[cfg(test)]
mod tests {

    use super::*;

    fn example_data() -> Vec<String> {
        vec![
            "London to Dublin = 464".to_owned(),
            "London to Belfast = 518".to_owned(),
            "Dublin to Belfast = 141".to_owned(),
        ]
    }


    #[test]
    fn parse_test() {

        let data = example_data();


        let data = parse_data(&data);

        assert!(data.is_ok());

        let (paths, cities) = data.unwrap();

        println!("{:#?}", paths);
        println!("{:#?}", cities);

        println!("-----");

        let permutations = cities.iter().permutations(cities.len());

        for p in permutations {
            println!("{:#?}", p);
        }

    }


    #[test]
    fn example() {
        let data = example_data();
        assert_eq!(run_1(&data).unwrap().0, 605);
    }

}