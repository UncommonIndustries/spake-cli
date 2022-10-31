use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use std::path::Path;

use clap::Parser;

mod file;
mod translate;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value = "strings/strings_en.json")]
    path: Option<String>,

    #[arg(short, long)]
    api_key: Option<String>,

    #[arg(short, long, default_value = "es")]
    target_language: Option<String>,
}

fn main() {
    let args = Args::parse();

    let source_path = args.path.unwrap();
    if !(Path::new(&source_path).exists()) {
        println!("Provided Filepath does not exist");
        return;
    }

    let target_language_argument = args.target_language.unwrap();
    let target_file_path = format!("strings/strings_{}.json", target_language_argument);

    let target_language = match translate::ValidTargetLanguages::from_str(&target_language_argument)
    {
        Ok(language) => language,
        Err(error) => {
            println!("target language not supported{}", error);
            return;
        }
    };
    let source_language = translate::ValidSourceLanguages::en;

    let json = match file::get_json(source_path) {
        Ok(json) => json,
        Err(error) => {
            println!("Error reading json file: {}", error);
            return;
        }
    };

    let mut destination_hash_map: HashMap<String, file::Key> = HashMap::new();

    for (key, value) in json.iter() {
        let request = translate::TranslationRequest {
            text: value.string.clone(),
            from_language: source_language,
            to_language: target_language,
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
    let target_file = target_file_path.to_string();
    let json = match serde_json::to_string_pretty(&destination_hash_map) {
        Ok(json) => json,
        Err(error) => {
            println!("Internat Error converting json to string: {}", error);
            return;
        }
    };

    let success = fs::write(target_file, json);
    match success {
        Ok(_) => println!("Successfully wrote to file"),
        Err(error) => println!("Error writing to file: {}", error),
    }
}
