use serde_json::{Value as JSONValue, Map as JSONObject, from_str as json_from_str};


fn recursive_count(
    json: &JSONValue,
    validate_obj: fn(&JSONObject<String, JSONValue>) -> bool,
) -> Result<i64, &'static str> {
    match json {
        JSONValue::Number(n) => {
            if n.is_i64() { Ok(n.as_i64().unwrap()) }
            else { Err("Floating point numbers are not supported.") }
        },
        JSONValue::Array(vec) => {
            let mut sum = 0;
            for val in vec { sum += recursive_count(val, validate_obj)?; }
            Ok(sum)
        },
        JSONValue::Object(map) => {
            let mut sum = 0;
            if validate_obj(map) {
                for val in map.values() { sum += recursive_count(val, validate_obj)?; }
            }
            Ok(sum)
        }
        _ => Ok(0)
    }
}


pub fn run_1(json: &str) -> Result<i64, &'static str> {
    let json: JSONValue = json_from_str(json).or(Err("Malformed JSON."))?;
    recursive_count(&json, |_| true)
}


pub fn run_2(json: &str) -> Result<i64, &'static str> {
    let json: JSONValue = json_from_str(json).or(Err("Malformed JSON."))?;
    recursive_count(&json, |map| {
        // When validating objects, double check that none of the object's values are "red"
        !map.values().any(|val| {
            if let JSONValue::String(s) = val { s == "red" } else { false }
        })
    })
}



#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    #[test_case(              r#"[1,2,3]"#, 6; "case 1")]
    #[test_case(        r#"{"a":2,"b":4}"#, 6; "case 2")]
    #[test_case(              r#"[[[3]]]"#, 3; "case 3")]
    #[test_case( r#"{"a":{"b":4},"c":-1}"#, 3; "case 4")]
    #[test_case(         r#"{"a":[-1,1]}"#, 0; "case 5")]
    #[test_case(         r#"[-1,{"a":1}]"#, 0; "case 6")]
    #[test_case(                   r#"[]"#, 0; "case 7")]
    #[test_case(                   r#"{}"#, 0; "case 8")]
    fn example_1(input: &str, expected: i64) {
        assert_eq!(run_1(input).unwrap(), expected);
    }

    #[test_case(                        r#"[1,2,3]"#, 6; "case 1")]
    #[test_case(        r#"[1,{"c":"red","b":2},3]"#, 4; "case 2")]
    #[test_case(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0; "case 3")]
    #[test_case(                    r#"[1,"red",5]"#, 6; "case 4")]
    fn example_2(input: &str, expected: i64) {
        assert_eq!(run_2(input).unwrap(), expected);
    }

}