use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Error;

use std::collections::HashMap;
use std::collections::LinkedList;
use std::fs::File;
use std::io::prelude::*;

// File.rs is designed to deal with reading and writing string files.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Key {
    pub string: String, // the value of the string that goes into a component
    pub example_keys: Option<LinkedList<HashMap<String, String>>>, // example keys that go into the string when it's valid
    pub translate: Option<bool>, // whether or not the string should be translated
}

pub fn from_json(filepath: String) -> Result<HashMap<String, Key>, Error> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let json: HashMap<String, Key> = serde_json::from_str(&contents)?;
    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_json() {
        let json = from_json("./tests/test.json".to_string()).unwrap();
        assert_eq!(json["test_key"].string, "test_value");
    }
    #[test]
    fn test_to_be_skipped() {
        let json = from_json("./tests/test.json".to_string()).unwrap();

        assert_eq!(json["ToBeSkipped"].translate, Some(false));
    }
}
