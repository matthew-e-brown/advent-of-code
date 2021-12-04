fn increment(password: &str) -> Result<String, &'static str> {

    let mut password: Vec<_> = password.chars().collect();
    let mut i = password.len() - 1;

    loop {
        let c = &mut password[i];
        if *c == 'z' {
            *c = 'a';
            if i == 0 {
                return Err("Password increment overflow ('zzzz' cannot be incremented)");
            } else {
                i -= 1;
            }
        } else {
            *c = char::from_u32(*c as u32 + 1).unwrap();
            break;
        }
    }

    Ok(password.into_iter().collect())
}


/// Increments a letter and resets everything to the right of it back to 'a', as if all the iterations were run. For
/// example, ('abcizxy', 3) would result in 'abcjaaa', as if all the iterations of the 'zxy' chunk were run through.
fn skip_increment(password: &str, index: usize) -> Result<String, &'static str> {
    // Take the string up to that index and increment it as normal,
    let former = increment(&password[..=index])?;

    // And then rebuild the second half out of just 'a'
    let length = password.len() - index - 1;
    let latter: String = std::iter::repeat('a').take(length).collect();

    Ok(former + &latter)
}


fn is_valid(password: &str) -> bool {

    fn increasing_straight(password: &str) -> bool {
        password
            .chars()
            .collect::<Vec<_>>()
            .windows(3)
            .any(|chars| {
                let a = chars[0] as u32;
                let b = chars[1] as u32;
                let c = chars[2] as u32;

                c == b + 1 && c == a + 2
            })
    }

    fn no_disallowed(password: &str) -> bool {
        !password
            .chars()
            .any(|c| c == 'i' || c == 'l' || c == 'o')
    }

    fn two_pairs(password: &str) -> bool {
        let mut found_first = false;
        let mut found_index = 0;

        // Loop through all the windows, keeping track of where we hit the first one
        for (i, window) in password.chars().collect::<Vec<_>>().windows(2).enumerate() {
            if window[0] == window[1] {

                #[cfg(test)]
                println!("i = {}, c = {}", i, window[0]);

                if !found_first {
                    found_first = true;
                    found_index = i;
                } else {
                    // This is the second pair, double check they don't overlap before returning true
                    if i > found_index + 1 { return true; }
                }
            }
        }

        false
    }


    two_pairs(password) && increasing_straight(password) && no_disallowed(password)
}


pub fn run(password: &str) -> Result<String, &'static str> {

    if password.len() != 8 {
        return Err("A password must be 8 characters long.");
    } else if !password.is_ascii() {
        return Err("Only ASCII passwords are supported.");
    } else if password.chars().any(|c| match c { 'a'..='z' => false, _ => true, }) {
        return Err("Passwords must be all lowercase.");
    }

    let res = Err("A valid password could not be found.");
    let mut password = increment(password).or(res.clone())?;

    while !is_valid(&password) {
        password = increment(&password).or(res.clone())?;

        // We can save a ton of time by skipping iterations that contain 'i', 'o', or 'l'
        if let Some(i) = password.chars().position(|c| c == 'i' || c == 'o' || c == 'l') {
            password = skip_increment(&password, i).or(res.clone())?;
        }

        #[cfg(test)]
        println!("{}", password);
    }

    Ok(password)
}


#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;


    #[test_case(    "a",     "b"; "case 1")]
    #[test_case(   "dz",    "ea"; "case 2")]
    #[test_case("mzzzz", "naaaa"; "case 3")]
    fn test_increment(start: &str, expected: &str) {
        assert_eq!(increment(start).unwrap(), expected);
    }


    #[test_case("hijklmmn", false; "case 1")]
    #[test_case("abbceffg", false; "case 2")]
    #[test_case("abbcegjk", false; "case 3")]
    fn test_valid(password: &str, expected: bool) {
        assert_eq!(is_valid(password), expected);
    }


    #[test_case("abcdefgh", "abcdffaa"; "case 1")]
    #[test_case("ghijklmn", "ghjaabcc"; "case 2")]
    fn examples(start: &str, expected: &str) {
        assert_eq!(run(start).unwrap(), expected);
    }

}