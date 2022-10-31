use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::Path;

use clap::Parser;

mod file;
mod translate;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    path: Option<String>,

    #[arg(short, long)]
    api_key: Option<String>,
}

fn get_file_path_from_defaults() -> Result<String, Error> {
    const DEFAULT_PATH: &str = "strings/strings_en.json";
    if Path::new(DEFAULT_PATH).exists() {
        Ok(DEFAULT_PATH.to_string())
    } else {
        Err(Error::new(std::io::ErrorKind::NotFound, "File not found"))
    }
}

fn main() {
    let args = Args::parse();
    let source_path;
    if args.path == None {
        let path = get_file_path_from_defaults();
        if path.is_err() {
            println!("No path provided and no default file found.");
            return;
        }
        source_path = path.unwrap();
    } else {
        source_path = args.path.unwrap();
        if !Path::new(&source_path).exists() {
            println!("File not found.");
            return;
        }
    }

    let json = file::get_json(source_path).unwrap();

    let mut destination_hash_map: HashMap<String, file::Key> = HashMap::new();

    for (key, value) in json.iter() {
        let targetLanguage = translate::ValidTargetLanguages::es;
        let sourceLanguage = translate::ValidSourceLanguages::en;

        let request = translate::TranslationRequest {
            text: value.string.clone(),
            from_language: sourceLanguage,
            to_language: targetLanguage,
        };
        let translation_result = translate::translate_string(request);
        println!("{:?}", translation_result);
        destination_hash_map.insert(
            key.clone(),
            file::Key {
                string: translation_result.unwrap().text,
                example_keys: None,
            },
        );
    }
    let target_file = "tests/strings_es.json".to_string();
    let json = serde_json::to_string_pretty(&destination_hash_map).unwrap(); // TODO remove unwrap
    fs::write(target_file, json);
}
