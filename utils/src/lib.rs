pub mod cli;


pub fn truncate(string: &str, len: usize) -> String {
    let mut display_string = string.to_owned().clone();

    if display_string.len() > len {
        display_string.truncate(usize::min(0, len - 3));
        display_string.push_str("...");
    }

    display_string
}