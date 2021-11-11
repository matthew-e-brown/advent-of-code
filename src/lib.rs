use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{App, Arg, ArgMatches};


pub fn generate_app_template<'a, 'b>(name: &'a str, about: &'b str) -> App<'a, 'b>
where
    'a: 'static,
    'b: 'static
{

    App::new(name)
        .about(about)
        .arg(
            Arg::with_name("part")
            .help("Which part of the AoC challenge to run")
            .long("part")
            .short("p")
            .possible_values(&[ "1", "2" ])
            .default_value("1")
        )
        .arg(
            Arg::with_name("inputs")
            .help("One or more inputs to test")
            .multiple(true)
            .required_unless("files")
        )
        .arg(
            Arg::with_name("files")
            .help("A file to read one or more inputs from (one input per line)")
            .long("file")
            .short("f")
            .multiple(true)
            .value_name("PATH")
            .required_unless("inputs")
        )

}


pub fn parse_matches<T>(matches: &ArgMatches) -> Vec<Result<String, &'static str>> {

    let mut lines = Vec::new();

    // Grab raw lines
    if let Some(inputs) = matches.values_of("inputs") {
        lines.extend(inputs.map(|s| Ok(s.to_owned())));
    }

    // Open and read lines from files
    if let Some(files) = matches.values_of("files") {
        'f: for file in files {

            // Open file as raw handle
            let handle = match File::open(file) {
                Ok(fd) => fd,
                Err(_) => {
                    lines.push(Err("Error opening file."));
                    continue;
                }
            };

            // Read its lines one by one
            let reader = BufReader::new(handle);

            for line in reader.lines() {
                match line {
                    Ok(string) => lines.push(Ok(string)),
                    Err(_) => {
                        lines.push(Err("Error reading from file"));
                        continue 'f;
                    }
                }
            }

            // Done
        }
    }

    lines
}