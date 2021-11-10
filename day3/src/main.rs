use std::{fs::File, io::{BufRead, BufReader}};
use clap::{App, load_yaml};

use day3::{Direction, directions_from_string, run_1, run_2};

struct ParsePair {
    string: String,
    result: Result<Vec<Direction>, &'static str>,
}


fn parse_args(matches: &clap::ArgMatches) -> Vec<ParsePair> {
    let mut pairs = Vec::new();

    // Parse raw input lines
    if let Some(inputs) = matches.values_of("inputs") {
        pairs.extend(inputs.map(|s| {
            let result = directions_from_string(s);
            ParsePair { string: s.to_owned(), result }
        }));
    }

    // Parse from files
    if let Some(files) = matches.values_of("files") {
        'outer: for file in files {
            let handle = File::open(file);

            if let Err(_) = handle {
                pairs.push(ParsePair { string: file.to_owned(), result: Err("Error opening file.") });
                continue;
            }

            let handle = handle.unwrap();
            let reader = BufReader::new(handle);

            for line in reader.lines() {
                match line {
                    Ok(string) => {
                        let result = directions_from_string(&string);
                        pairs.push(ParsePair { string, result });
                    },
                    Err(_) => {
                        pairs.push(ParsePair { string: file.to_owned(), result: Err("Error reading file.") });
                        continue 'outer;
                    }
                };
            }
        }
    }

    pairs
}


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let pairs = parse_args(&matches);

    if pairs.len() < 1 {
        panic!("Program ran without any args!");
    }

    for mut pair in pairs {
        match pair.result {
            Ok(sequence) => {
                let part: u8 = if let Some(part) = matches.value_of("part") {
                    part.parse().unwrap()
                } else {
                    1
                };

                let output = if part == 1 { run_1(&sequence) } else { run_2(&sequence) };
                if pair.string.len() > 12 {
                    pair.string.truncate(9);
                    pair.string.push_str("...");
                }

                println!("Sequence {:>12}: results in {} houses getting presents.", pair.string, output);
            },
            Err(e) => eprintln!("{}: {}", pair.string, e),
        }
    }
}