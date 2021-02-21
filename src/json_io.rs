use std::{
    fs,
    fs::File,
    io::{
        prelude::*,
        ErrorKind
    },
};
use serde_json::{
    Result,
    Value,
    json
};

pub fn read_from_output() -> std::io::Result<Value> {
    let mut string_buffer = String::new();

    match File::open(format!("./py_io/output.json")) {
        Ok(mut file) => {
            match file.read_to_string(&mut string_buffer) {
                Ok(_) => {
                    match serde_json::from_str(string_buffer.as_str()) {
                        Ok(value) => { return Ok(value); },
                        Err(err) => { return Err(std::io::Error::new(ErrorKind::Other, "unable to construct json from file")); },
                    }
                },
                Err(err) => { return Err(err); },
            };
        },
        Err(err) => { return Err(err); },
    };
}

pub fn write_to_input(json: Value) -> std::io::Result<()> {
    let json_string = json.to_string();
    let json_bytes = json_string.as_bytes();

    match File::create("./py_io/input.json") {
        Ok(mut file) => {
            match file.write_all(json_bytes) {
                Ok(()) => { return Ok(()); },
                Err(err) => { return Err(err); }
            }
        },
        Err(err) => { return Err(err); },
    };
}

#[cfg(test)]
mod json_io_test {
    use serde_json::{Result, Value, json};
    use super::*;

    #[test]
    fn simple_write() -> () {
        let json = json!({
            "x": 34,
            "y": "a string",
            "z": [3,4,5],
        });

        let result = write_to_input(json);

        match result {
            Ok(()) => (),
            Err(e) => { panic!(); },
        }
    }

    #[test]
    fn complex_write() -> () {
        let json = json!({
            "x": 34,
            "obj": {
                "asdf": true,
            },
        });

        let result = write_to_input(json);

        match result {
            Ok(()) => (),
            Err(e) => { panic!(); },
        }
    }

    #[test]
    fn simple_read() -> () {
        match read_from_output() {
            Ok(json) => (),
            Err(err) => { panic!(); }
        };
    }
}