fn look_and_say(string: &String) -> Result<String, &'static str> {

    let mut output = vec![];

    // Get the chars and seed the first one
    let mut iter = string.chars().peekable();
    let mut current_c = *iter.peek().ok_or("String needs at least one character")?;
    let mut current_n = 0usize;

    while let Some(c) = iter.next() {
        if c == current_c {
            current_n += 1;
        } else {
            // Add to our output (use a string intermediate to handle cases where n > 9)
            output.extend(current_n.to_string().chars());
            output.push(current_c);

            // re-seed the loop
            current_c = c;
            current_n = 1;
        }
    }

    // Push the final char and its count onto the string
    output.extend(current_n.to_string().chars());
    output.push(current_c);

    Ok(output.into_iter().collect())
}


pub fn run(string: &String, iterations: usize) -> Result<String, &'static str> {
    let mut i = 0;
    let mut string = string.clone();
    loop {
        string = look_and_say(&string)?;

        #[cfg(test)]
        println!("{}", string);

        i += 1;
        if i >= iterations { break Ok(string); }
    }
}



#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    #[test]
    fn example() {
        let example = "1".to_owned();
        assert_eq!(run(&example, 5).unwrap(), "312211")
    }


    #[test_case(    "123",     "111213"; "case 1")]
    #[test_case("5540042", "2514201412"; "case 2")]
    fn basic_test(input: &str, expected: &str) {
        let example = input.to_owned();
        assert_eq!(look_and_say(&example).unwrap(), expected);
    }


    #[test_case(    "123", 3,                         "1321123113"; "case 1")]
    #[test_case("5540042", 4, "3112311513211431123110132114132112"; "case 2")]
    fn iterative_test(input: &str, num: usize, expected: &str) {
        let example = input.to_owned();
        assert_eq!(run(&example, num).unwrap(), expected);
    }

    // 123
    // 11 12 13        =      111213
    // 31 12 11 13     =    31121113
    // 13 21 12 31 13  =  1321123113

    // 5540042
    // 25 14 20 14 12                                      =                          2514201412
    // 12 15 11 14 12 10 11 14 11 12                       =                12151114121011141112
    // 11 12 11 15 31 14 11 12 11 10 31 14 31 12           =        1112111531141112111031143112
    // 31 12 31 15 13 21 14 31 12 31 10 13 21 14 13 21 12  =  3112311513211431123110132114132112

}