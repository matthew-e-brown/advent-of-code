use std::collections::HashMap;
use std::{ops::{Deref, DerefMut}, fmt};
use regex::Regex;
use itertools::Itertools;

// neighbour -> happiness change
type Opinions<'a> = HashMap<&'a str, isize>;
// name -> (neighbour -> happiness change) map
type GuestsInner<'a> = HashMap<&'a str, Opinions<'a>>;


#[derive(Clone)]
pub struct TableGuests<'a>(GuestsInner<'a>);

impl<'a> Deref for TableGuests<'a> {
    type Target = GuestsInner<'a>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'a> DerefMut for TableGuests<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<'a> fmt::Debug for TableGuests<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, (guest, opinions)) in self.iter().enumerate() {

            let longest = {
                let mut m = 0;
                for name in opinions.keys() {
                    let l = name.len();
                    if l > m { m = l; }
                }
                m
            };

            writeln!(f, "Guest '{}' with opinions:", guest)?;
            for (neighbour, change) in opinions.iter() {
                let padding = longest - neighbour.len();
                let sign = if *change < 0 { "-" } else { "+" };

                write!(f, "\tNeighbour '{}':", neighbour)?;
                writeln!(f, "{:>p$}  {} {}", "", sign, change.abs(), p=padding)?;
            }
            if i < self.len() - 1 { write!(f, "\n")?; }
        }

        Ok(())
    }
}


fn validate_opinions(table: &TableGuests) -> bool {
    // For every guest, check that they have an entry for every other guest
    for (guest, opinions) in table.iter() {
        for neighbour in table.keys() {
            if guest == neighbour { continue; }
            if !opinions.contains_key(neighbour) { return false; }
        }
    }

    true
}


pub fn create_table(data: &Vec<String>) -> Result<TableGuests, String> {

    if data.len() < 2 {
        return Err("There must be at least two guests at the table.".to_owned());
    }
    
    let re = Regex::new(r"^(\w+).+(gain|lose) (\d+).+next to (\w+)\.?$").unwrap();
    let mut guests = GuestsInner::new();

    for line in data.iter() {
        let caps = re.captures(line).ok_or(format!("Malformed line: {}", line))?;

        // Because regex uses anchors, we know we will always have the groups we want if we get this far

        let name = caps.get(1).unwrap().as_str();
        let neighbour = caps.get(4).unwrap().as_str();
        let amount = {
            // check 'gain' or 'lose'
            let multiplier = if caps.get(2).unwrap().as_str() == "gain" { 1 } else { -1 };
            caps.get(3).unwrap().as_str().parse::<isize>().unwrap() * multiplier
        };

        if let Some(current_opinions) = guests.get_mut(name) {
            // If this guest already has a map being built up of their neighbours...
            // Insert the new neighbour, and fail on duplicates
            if let Some(old) = current_opinions.insert(neighbour, amount) {
                return Err(format!(
                    "Duplicate neighbour for '{}': neighbour '{}' as both {} and {}.",
                    name, neighbour, old, amount
                ));
            }
        } else {
            let mut new_opinions = HashMap::new();
            new_opinions.insert(neighbour, amount);
            guests.insert(name, new_opinions);
        }
    }

    let result = TableGuests { 0: guests };

    if !validate_opinions(&result) {
        Err("Table is not valid: all guests must have an opinion of all other guests.".to_owned())
    } else {
        Ok(result)
    }
}


pub fn run_1<'a, 'b>(table: &'a TableGuests<'b>) -> (isize, Vec<&'b str>) {
    let mut highest_net = 0;
    let mut highest_vec = None;
    let guest_orders = table.keys().permutations(table.len());

    for permutation in guest_orders {
        let mut net = 0;

        for (&left, &guest, &right) in permutation.iter().circular_tuple_windows() {
            let opinions = table.get(guest).unwrap();
            net += opinions.get(left).unwrap();
            net += opinions.get(right).unwrap();
        }

        #[cfg(test)]
        println!("Permutation:\t{:?}\nNet change:\t{}\n", permutation, net);

        if net > highest_net {
            highest_net = net;
            highest_vec = Some(permutation.iter().map(|r| **r).collect());
        }
    }

    (highest_net, highest_vec.unwrap())
}


pub fn run_2<'a, 'b>(table: &'a TableGuests<'b>) -> (isize, Vec<&'b str>) {

    // Insert myself at the table. To do so, create a new table that we can modify
    let mut table = table.clone();
    let mut my_opinions = Opinions::new();

    for (guest, their_opinions) in table.iter_mut() {
        my_opinions.insert(*guest, 0);
        their_opinions.insert("[Me]", 0);
    }

    table.insert("[Me]", my_opinions);

    run_1(&table)
}



#[cfg(test)]
mod tests {

    use super::*;

    fn example_data() -> Vec<String> {
        vec![
            "Alice would gain 54 happiness units by sitting next to Bob.".to_owned(),
            "Alice would lose 79 happiness units by sitting next to Carol.".to_owned(),
            "Alice would lose 2 happiness units by sitting next to David.".to_owned(),
            "Bob would gain 83 happiness units by sitting next to Alice.".to_owned(),
            "Bob would lose 7 happiness units by sitting next to Carol.".to_owned(),
            "Bob would lose 63 happiness units by sitting next to David.".to_owned(),
            "Carol would lose 62 happiness units by sitting next to Alice.".to_owned(),
            "Carol would gain 60 happiness units by sitting next to Bob.".to_owned(),
            "Carol would gain 55 happiness units by sitting next to David.".to_owned(),
            "David would gain 46 happiness units by sitting next to Alice.".to_owned(),
            "David would lose 7 happiness units by sitting next to Bob.".to_owned(),
            "David would gain 41 happiness units by sitting next to Carol.".to_owned()
        ]
    }


    #[test]
    fn parse() {
        let data = example_data();
        let table = create_table(&data).unwrap();
        println!("{:?}", table);
    }


    #[test]
    fn example() {
        let data = example_data();
        let table = create_table(&data).unwrap();
        assert_eq!(run_1(&table).0, 330);
    }

}