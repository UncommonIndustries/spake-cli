use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use std::path::Path;

use clap::{Args, Parser, Subcommand};

mod file;
mod params;
mod translate;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Translate(TranslateArgs),
}

#[derive(Args)]
struct TranslateArgs {
    #[arg(short, long, default_value = "strings/strings_en.json")]
    path: Option<String>,

    #[arg(short, long)]
    api_key: Option<String>,

    #[arg(short, long, default_value = "es")]
    target_language: Option<String>,

    #[arg(short, long, default_value = "en")]
    source_language: Option<String>,

    #[arg(long, default_value=params::PRODUCTION_ENDPOINT)]
    host: Option<String>,
}

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Commands::Translate(args) => {
            let source_path = args.path.as_ref().unwrap();
            if !(Path::new(&source_path).exists()) {
                println!("Provided filepath does not exist");
                return;
            }

            let target_language_argument = args.target_language.as_ref().unwrap();
            let target_file_path = format!("strings/strings_{}.json", target_language_argument);

            let target_language =
                match translate::ValidTargetLanguages::from_str(&target_language_argument) {
                    Ok(language) => language,
                    Err(error) => {
                        println!("target language not supported{}", error);
                        return;
                    }
                };
            let source_language = match translate::ValidSourceLanguages::from_str(
                args.source_language.as_ref().unwrap(),
            ) {
                Ok(language) => language,
                Err(error) => {
                    println!("source language not supported: {}", error);
                    return;
                }
            };

            let json = match file::get_json(source_path.to_string()) {
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
                println!("Translating: {:?}...", request);
                let translation_result =
                    match translate::translate_string(request, args.host.clone().unwrap()) {
                        Ok(translation) => translation,
                        Err(error) => {
                            println!("Error translating string: {}", error);
                            return;
                        }
                    };

                destination_hash_map.insert(
                    key.clone(),
                    file::Key {
                        string: translation_result.text,
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
    }
}
