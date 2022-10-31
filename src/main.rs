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

fn main() {
    let args = Args::parse();
    // TODO flesh out the args again.

    let filePath = "tests/strings_en.json".to_string();
    let json = file::get_json(filePath).unwrap();

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
    }
}
