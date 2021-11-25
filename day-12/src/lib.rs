use serde_json::{Value as JSONValue, from_str as json_from_str};


fn recursive_count(json: &JSONValue) -> Result<i64, &'static str> {
    match json {
        JSONValue::Number(n) => {
            if n.is_i64() { Ok(n.as_i64().unwrap()) }
            else { Err("Floating point numbers are not supported.") }
        },
        JSONValue::Array(vec) => {
            let mut sum = 0;
            for val in vec { sum += recursive_count(val)?; }
            Ok(sum)
        },
        JSONValue::Object(map) => {
            let mut sum = 0;
            for val in map.values() { sum += recursive_count(val)?; }
            Ok(sum)
        }
        _ => Ok(0)
    }
}


pub fn run(json: &str) -> Result<i64, &'static str> {
    let json: JSONValue = json_from_str(json).or(Err("Malformed JSON."))?;
    recursive_count(&json)
}