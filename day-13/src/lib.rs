use std::collections::HashMap;
use std::{ops::{Deref, DerefMut}, fmt};
use regex::Regex;

// neighbour -> happiness change
type Opinions<'a> = HashMap<&'a str, isize>;
// name -> (neighbour -> happiness change) map
type GuestsInner<'a> = HashMap<&'a str, Opinions<'a>>;

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


pub fn create_table(lines: &Vec<String>) -> Result<TableGuests, String> {

    let re = Regex::new(r"^(\w+).+(gain|lose) (\d+).+next to (\w+)\.?$").unwrap();

    let mut guests: GuestsInner = HashMap::new();

    for line in lines.iter() {
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

    Ok(TableGuests{ 0: guests })
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
        println!("{:#?}", table);
    }

}