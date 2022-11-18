use std::collections::HashMap;
use std::fs;

use std::path::Path;

use clap::{Args, Parser, Subcommand};

use futures::{stream, StreamExt};

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
    #[arg(short, long, default_value = "src/strings/strings_en.json")]
    path: Option<String>,

    #[arg(short, long, env = "SPAKE_API_KEY")]
    api_key: String,

    #[arg(short, long, default_value = "es")]
    target_language: Option<translate::models::ValidTargetLanguages>,

    #[arg(short, long, default_value = "en")]
    source_language: Option<translate::models::ValidSourceLanguages>,

    #[arg(long, default_value=params::PRODUCTION_ENDPOINT)]
    host: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Commands::Translate(args) => {
            let source_path = args.path.as_ref().unwrap();
            if !(Path::new(&source_path).exists()) {
                println!("Provided filepath does not exist");
                return;
            }

            let target_language_argument = args.target_language.as_ref().unwrap();
            let target_file_path = format!("strings/strings_{:#?}.json", target_language_argument);

            let target_language = target_language_argument;
            let source_language = args.source_language.as_ref().unwrap();

            let api_key = &args.api_key;

            let json = match file::get_json(source_path.to_string()) {
                Ok(json) => json,
                Err(error) => {
                    println!("Error reading json file: {}", error);
                    return;
                }
            };

            let translation_result = stream::iter(json)
                .map(|(key, value)| {
                    let key = key.clone();
                    let value = value.clone();
                    async move {
                        let translation_request =
                            translate::translation_request::TranslationRequest {
                                text: value.string.clone(),
                                from_language: *source_language,
                                to_language: *target_language,
                            };

                        let translation = translate::translate::translate_string(
                            translation_request,
                            args.host.clone().unwrap(),
                            api_key.clone(),
                        )
                        .await;
                        (key, translation)
                    }
                })
                .buffer_unordered(9);
            let destination_hash_map: HashMap<String, file::Key> = HashMap::new();
            let q = translation_result
                .fold(
                    destination_hash_map,
                    |mut destination_hash_map, (key, value)| async {
                        let translation = match value {
                            Ok(translation) => translation,
                            Err(error) => {
                                println!("Error translating string: {}", error);
                                return destination_hash_map;
                            }
                        };
                        let translated_result = file::Key {
                            string: translation.text,
                            example_keys: None,
                        };
                        destination_hash_map.insert(key, translated_result);
                        destination_hash_map
                    },
                )
                .await;

            let target_file = target_file_path.to_string();
            let json = match serde_json::to_string_pretty(&q) {
                Ok(json) => json,
                Err(error) => {
                    println!("Error converting json to string: {}", error);
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
