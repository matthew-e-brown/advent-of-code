use std::{env, process};
use day3::{Direction, directions_from_string, run};


struct Pair<'s> {
    as_string: &'s str,
    as_direct: Vec<Direction>,
}


fn process_args(args: &[String]) -> Result<Vec<Pair>, &str> {
    if args.len() < 2 {
        return Err("Please pass at least one sequence.");
    }

    args[1..].iter().map(|as_string| {
        match directions_from_string(as_string) {
            Ok(as_direct) => Ok(Pair { as_string, as_direct }),
            Err(e) => Err(e),
        }
    }).collect()
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let sequences = process_args(&args[..]).unwrap_or_else(|err| {
        eprintln!("Argument error: {}", err);
        process::exit(1);
    });

    let longest = sequences.iter().fold(0_usize, |acc, cur| {
        let len = cur.as_string.len();
        if len > acc { len } else { acc }
    });

    for sequence in sequences.iter() {
        let result = run(&sequence.as_direct);
        println!(
            "'{:>w$}' delivers presents to {} houses.",
            sequence.as_string, result, w = longest
        );
    }
}