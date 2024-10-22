//! A simple check  to prove if a JSON object contains a path and or value.
//! This code is for illustrative purposes only it should not be assumed to be correct.
#![no_main]

use entrypoint::io::InputTape;
use serde::Deserialize;
use serde_json::Value;

entrypoint::entrypoint!(main);

fn main() {
    let mut de = serde_json::Deserializer::from_reader(InputTape);
    let Input {
        path,
        json,
        expected,
    } = Input::deserialize(&mut de).expect("could not parse input");

    validate_json_path(&path, &expected, &json);
    entrypoint::io::println(&format!("path: {}", path));
    entrypoint::io::println(&format!("contains json: {}", expected));
}

#[derive(Deserialize)]
struct Input {
    path: String,
    json: Value,
    expected: Value,
}

fn validate_json_path(path: &str, expected: &Value, json: &Value) {
    let compiled = jsonpath_lib::Compiled::compile(path).expect("Invalid JSONPath expression");
    let results = compiled.select(json).expect("Failed to evaluate JSONPath");

    match expected {
        Value::Array(expected) => {
            assert_eq!(results.len(), expected.len());
            for (result, expected) in results.iter().zip(expected.iter()) {
                assert_eq!(result, &expected);
            }
        }
        value => {
            assert_eq!(results.len(), 1);
            assert_eq!(results[0], value);
        }
    }
}
