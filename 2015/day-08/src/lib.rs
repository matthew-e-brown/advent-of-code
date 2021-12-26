pub fn run_1(strings: &Vec<String>) -> usize {

    let mut total_raw = 0;
    let mut total_mem = 0;

    for string in strings {

        // Start and end quote, cover case with malformed string just b'cuz
        if string.starts_with(r#"""#) { total_raw += 1; }
        if string.ends_with(r#"""#) { total_raw += 1; }

        let mut chars = string.chars();

        while let Some(c) = chars.next() {
            total_raw += 1;
            total_mem += 1;

            if c == '\\' {

                // If it's a \\, then that means we have one fewer in-memory character than in-text.  
                // If it's a \", then the same applies.  
                // If it's a \x00 sequence, then that means we have three fewer.  

                if let Some(next) = chars.next() {
                    if next == '\\' || next == '"' {
                        total_mem -= 1;
                    } else if next == 'x' {
                        total_mem -= 3;
                    }
                }
            }
        } // End while

    }

    total_raw - total_mem
}


pub fn run_2(strings: &Vec<String>) -> usize {

    let mut total_raw = 0;
    let mut total_enc = 0;

    for string in strings {

        // Include wrapper quotes
        total_enc += 2;

        let mut chars = string.chars();

        while let Some(c) = chars.next() {
            total_raw += 1;
            total_enc += 1;
            if c == '\\' || c == '"' { total_enc += 1; }
        }

    }

    total_enc - total_raw
}


#[cfg(test)]
mod tests {

    use super::*;


    fn example_data() -> Vec<String> {
        vec![
            r#""""#.to_owned(),
            r#""abc""#.to_owned(),
            r#""aaa\"aaa""#.to_owned(),
            r#""\x27""#.to_owned(),
        ]
    }


    #[test]
    fn example_1() {
        let data = example_data();
        assert_eq!(run_1(&data), 12);
    }


    #[test]
    fn example_2() {
        let data = example_data();
        assert_eq!(run_2(&data), 19);
    }

}