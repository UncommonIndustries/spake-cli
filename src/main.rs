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
    if let Some(filepath) = args.path {
        let json = file::get_json(filepath);
        // println!("{:?}", json);
    }
    let request = translate::TranslationRequest {
        text: "my name is { user_name }.".to_string(),
        from_language: "en".to_string(), //translate::ValidTargetLanguages::es,
        to_language: "es".to_string(),   //translate::ValidSourceLanguages::en,
    };
    translate::translate_string(request);
}
