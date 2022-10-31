use clap::Parser;
mod file;

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
        println!("{:?}", json);
    }
}
