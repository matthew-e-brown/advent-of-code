use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{App, Arg, ArgMatches};


pub struct Config {
    pub part: u8,
    pub data: Vec<Result<String, &'static str>>
}


pub fn parse_cli<'a, 'b>(name: &'a str, about: &'b str) -> Config
where
    'a: 'static,
    'b: 'static,
{

    let app = generate_clap_template(name, about);
    let matches = app.get_matches();

    // 'part' has a default value of "1" and can only be one of&["1", "2"], so we unwrap and parse it
    let part = matches.value_of("part").unwrap().parse().unwrap();
    let data = parse_matches(&matches);

    Config { part, data }
}


fn generate_clap_template<'a, 'b>(name: &'a str, about: &'b str) -> App<'a, 'b>
where
    'a: 'static,
    'b: 'static
{

    App::new(name)
        .about(about)
        .author("Matthew Brown <matthew.e.brown.17@gmail.com>")
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


fn parse_matches(matches: &ArgMatches) -> Vec<Result<String, &'static str>> {

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