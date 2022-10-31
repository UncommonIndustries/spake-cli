use std::collections::HashMap;
use std::fs;

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

    #[arg(short, long)]
    target_language: Option<String>,
}

fn main() {
    let args = Args::parse();

    let source_path = args.path.unwrap();
    if !(Path::new(&source_path).exists()) {
        println!("Provided Filepath does not exist");
        return;
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
