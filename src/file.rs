use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Error;

use std::collections::HashMap;
use std::collections::LinkedList;
use std::fs::File;
use std::io::prelude::*;

// File.rs is designed to deal with reading and writing string files.
#[derive(Serialize, Deserialize, Debug)]
pub struct Key {
    string: String, // the value of the string that goes into a component
    example_keys: Option<LinkedList<HashMap<String, String>>>, // example keys that go into the string when it's valid
}

pub fn get_json(filepath: String) -> Result<HashMap<String, Key>, Error> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let json: HashMap<String, Key> = serde_json::from_str(&contents)?;
    Ok(json)
}
