use std::collections::{HashMap, HashSet};
use std::fs;

use std::io::Write;
use std::path::Path;

use clap::{Args, Parser, Subcommand};

use futures::{stream, StreamExt};

use fancy_regex::Regex;
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

    #[arg(short, long, env = "SPAKE_API_KEY")]
    api_key: String,

    #[arg(long, default_value=params::PRODUCTION_ENDPOINT)]
    host: Option<String>,

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
    target_language: translate::models::ValidTargetLanguages,

    #[arg(short, long, default_value = "en")]
    source_language: translate::models::ValidSourceLanguages,

    #[arg(long, default_value=params::PRODUCTION_ENDPOINT)]
    host: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Commands::Translate(args) => {
            let source_filepath_str = args.path.as_ref().unwrap();
            let source_filepath = Path::new(&source_filepath_str);

            if !source_filepath.exists() {
                println!("Provided filepath does not exist");
                return;
            };

            let target_language = args.target_language;
            let source_language = args.source_language;

            let parent_dir = source_filepath.parent().unwrap();

            let target_filename = format!("strings_{:#?}.json", target_language);
            let target_filepath = parent_dir.join(target_filename);

            let api_key = &args.api_key;

            let source_json = match file::from_json(source_filepath_str.to_string()) {
                Ok(json) => json,
                Err(error) => {
                    println!("Error reading json file: {}", error);
                    return;
                }
            };
            // kind of a weird hack to do this;; we can preemptively identify which keys we should skip and skip them here.
            let mut destination_hash_map: HashMap<String, file::Key> = source_json.clone();
            let mut to_translate: HashMap<String, file::Key> = source_json.clone();

            // This needs to be here because translation_result moves the value, after which we cannot borrow
            let src_keys: HashSet<String> = to_translate.keys().cloned().collect();

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
                                from_language: source_language,
                                to_language: target_language,
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

            // Check that the keys match between source and dest, and if they don't, print out the extras
            let tgt_keys: HashSet<String> = q.keys().cloned().collect();

            if !src_keys.eq(&tgt_keys) {
                println!("Key mismatch between translation source and target");

                let src_diff = src_keys.difference(&tgt_keys);
                let tgt_diff = tgt_keys.difference(&src_keys);

                if src_diff.clone().count() > 0 {
                    println!("Extra keys in source: {:?}", src_diff);
                }

                if tgt_diff.clone().count() > 0 {
                    println!("Extra keys in target: {:?}", tgt_diff);
                }
            }

            let json = match serde_json::to_string_pretty(&q) {
                Ok(json) => json,
                Err(error) => {
                    println!("Error converting json to string: {}", error);
                    return;
                }
            };

            let success = fs::write(target_filepath, json);
            match success {
                Ok(_) => println!("Successfully wrote to file"),
                Err(error) => println!("Error writing to file: {}, ", error),
            }
        }
        Commands::Beta(beta) => match beta {
            Beta::Gather(args) => {
                // Gather should gather all the strings from the codebase and create a json file.
                let api_key = args.api_key.clone();
                let host = args.host.clone().unwrap();
                let source_directory = args.source_code_directory.clone().unwrap();
                let _strings_file_path = args.path.clone().unwrap();

                // 1) traverse the structure and find all the js or jsx files
                let mut string_literals: Vec<gather::gather::GatherResponseObject> = Vec::new();

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
                            let file_path = path.to_str().unwrap().to_string();
                            let mut result = match gather::gather::identify_strings_in_file(
                                file_path,
                                api_key.clone(),
                                host.clone(),
                            )
                            .await
                            {
                                Ok(res) => res,
                                Err(err) => {
                                    println!("Error doing something: {:?}", err);
                                    continue;
                                }
                            };
                            string_literals.append(&mut result);
                            println!("{:?}", string_literals.len())
                        }
                    }
                }

                // 3) create a json file with the strings and the keys
                let json_data_to_write = match serde_json::to_string_pretty(&string_literals) {
                    Ok(json) => json,
                    Err(error) => {
                        println!("Error writing result to file");
                        return;
                    }
                };

                let target_file = "./src/strings/gather_results.json".to_string();
                match fs::write(target_file, json_data_to_write) {
                    Ok(_) => println!("Successfully wrote to file"),
                    Err(error) => println!("error writing to file: {}", error),
                }
            }
            Beta::Init(args) => {
                // Init should create the appropriate strings folder and the base json file.
                // it should have an optional parameter for doing the gather step too.

                let full_path = Path::new(args.base_path.as_ref().unwrap());
                let strings_folder = full_path.parent();
                let strings_folder_string: &str;
                // create strings folder
                match strings_folder {
                    Some(parent_path) => strings_folder_string = parent_path.to_str().unwrap(),
                    None => {
                        println!("Error creating strings folder. Folder name is Bogus.");
                        return;
                    }
                }

                // validate file name
                let file_name = full_path.file_name();
                let file_name_string: &str;
                match file_name {
                    Some(file_name) => file_name_string = file_name.to_str().unwrap(),
                    None => {
                        println!("Error validating string name. FileName is bogus.");
                        return;
                    }
                }

                let re = Regex::new(r"^strings_[A-Za-z]{2}\.json$").unwrap();
                let matches = re.is_match(file_name_string);
                match matches {
                    Ok(is_valid) => {
                        if !is_valid {
                            println!("Error validating target strings name, {:?} does not conform to standard strings file format.", file_name_string);
                            return;
                        }
                    }
                    Err(_) => {
                        println!("Error validating strings name. Input is so garbage it breaks the regex module somehow.");
                        return;
                    }
                }

                let _ = fs::create_dir_all(strings_folder_string);
                let mut file = match fs::File::create(full_path) {
                    Ok(file) => file,
                    Err(err) => {
                        println!("Error creating spake-language file");
                        return;
                    }
                };
                let default_file_data = b"{}";
                match file.write_all(default_file_data) {
                    Ok(_) => {
                        println!("Spake initialized")
                    }
                    Err(_) => {
                        println!("Error initializing spake file, ");
                        return;
                    }
                }
            }
        },
    }
}
