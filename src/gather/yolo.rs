use std::fs;

use super::gather::GatherResponseObject;
use base64::{engine::general_purpose, Engine as _};

use crate::file;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn yolo_strings_into_files<'a>(gather_result: Vec<GatherResponseObject>) {
    for file in gather_result {
        let file_name = file.fileName;
        let file_components_len = file.components.len();
        let file_data = fs::read_to_string(file_name.clone()).unwrap();
        let mut file_data: Vec<String> = file_data.split("\n").map(String::from).collect();
        let mut component_line_offset = 0;
        for component in file.components {
            // replace inline strings with keys. hopefully.
            for string_literal in component.literals {
                let key_name = get_key_name(
                    file_name.clone(),
                    component.name.clone(),
                    string_literal.text.clone(),
                );
                let new_line = format!("{{ strings.{} }}", key_name);
                if string_literal.lineNumber.len() > 1 {
                    continue;
                }
                let line_number = (string_literal.lineNumber[0] - 1) as usize;
                if string_literal.text.trim() != file_data[line_number].trim() {
                    continue;
                }
                let line_data = &file_data[line_number];
                let left_padding = line_data.len() - line_data.trim_start().len();
                let left_string = " ".repeat(left_padding);
                let padded_new_line = left_string + &new_line;

                file_data[line_number] = padded_new_line;
                let strings_file_data =
                    file::from_json("./src/strings/strings_en.json".to_string()).unwrap();
                if strings_file_data.contains_key(key_name.as_str()) {
                    continue;
                }
                let new_key = file::Key {
                    string: string_literal.text.trim().to_string(),
                    example_keys: None,
                    translate: None,
                };
                let mut new_strings_file_data = strings_file_data;
                new_strings_file_data.insert(key_name, new_key);
                let json = match serde_json::to_string_pretty(&new_strings_file_data) {
                    Ok(json) => json,
                    Err(error) => {
                        println!("Error converting json to string: {}", error);
                        return;
                    }
                };
                let r = fs::write("./src/strings/strings_en.json".to_string(), json);
                match r {
                    Ok(_) => {}
                    Err(error) => {
                        println!("Error writing to file during yolo: {}", error);
                        return;
                    }
                }
            }
            // add use of strings package to each component
            let use_line = "  const { strings } = useSpakeState();";
            let line_number = (component.lineNumber + component_line_offset) as usize;
            file_data.insert(line_number, use_line.to_string());
            component_line_offset += 1;
        }
        if file_components_len > 0 {
            file_data.insert(
                0,
                "import { useSpakeState } from 'spake-react-sdk';".to_string(),
            );
        }

        let file_data = file_data.join("\n");
        fs::write(file_name, file_data).unwrap();
    }
}

fn get_key_name(file_name: String, component_name: String, text: String) -> String {
    let file_name = file_name
        .split("/")
        .last()
        .unwrap()
        .split(".")
        .next()
        .unwrap();

    let text = text.as_bytes();
    let hashed_text = hash(&text);
    let encoded = general_purpose::URL_SAFE_NO_PAD.encode(&hashed_text.to_be_bytes());

    let key_name = format!("{}_{}_{}", file_name, component_name, encoded);
    key_name
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
