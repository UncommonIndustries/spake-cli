use std::collections::HashMap;
use std::fs;

use std::path::Path;

use clap::{Args, Parser, Subcommand};

use futures::{stream, StreamExt};

use std::ffi::OsStr;
use walkdir::WalkDir;

mod file;
mod gather;
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

    #[command(subcommand)]
    Beta(Beta),
}

#[derive(Subcommand)]
enum Beta {
    Gather(GatherArgs),
    Init(InitArgs),
}

#[derive(Args)]
struct GatherArgs {
    #[arg(short, long, default_value = "src/strings/strings_en.json")]
    path: Option<String>,

    #[arg(short, long, default_value = "src/")]
    source_code_directory: Option<String>,
}

#[derive(Args)]
struct InitArgs {
    #[arg(short, long, default_value = "src/strings/strings_en.json")]
    base_path: Option<String>,
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

            let source_json = match file::get_json(source_path.to_string()) {
                Ok(json) => json,
                Err(error) => {
                    println!("Error reading json file: {}", error);
                    return;
                }
            };
            // kind of a weird hack to do this;; we can preemptively identify which keys we should skip and skip them here.
            let mut destination_hash_map: HashMap<String, file::Key> = source_json.clone();
            let mut to_translate: HashMap<String, file::Key> = source_json.clone();

            destination_hash_map.retain(|_, v| v.translate == Some(false));
            to_translate.retain(|_, v| v.translate == None || v.translate == Some(true));

            let translation_result = stream::iter(to_translate)
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
                            translate: None,
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
        Commands::Beta(beta) => match beta {
            Beta::Gather(args) => {
                // Gather should gather all the strings from the codebase and create a json file.

                let source_directory = args.source_code_directory.clone().unwrap();
                let strings_file_path = args.path.clone().unwrap();

                // 1) traverse the structure and find all the js or jsx files
                for entry in WalkDir::new(source_directory)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    let path = entry.path();
                    if path.is_file() {
                        let extension = path.extension();
                        if extension == Some(OsStr::new("js"))
                            || extension == Some(OsStr::new("jsx"))
                        {
                            println!("Found File: {:?}", path.to_str());
                            // 2) parse the files and find all the strings that are being passed to the translate function
                            gather::extractor::replace_raw_strings_in_file(
                                path.to_str().unwrap(),
                                &strings_file_path,
                            );
                            // TODO fix the unwrap here.
                        }
                    }
                }

                // 3) create a json file with the strings and the keys

                println!("Gathering strings");
            }
            Beta::Init(_) => {
                // Init should create the appropriate strings folder and the base json file.
                // it should have an optional parameter for doing the gather step too.

                println!("Not Implemented yet. Coming soon!");
            }
        },
    }
}
