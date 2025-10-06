use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "translator")]
#[command(about = "Translate words between languages using LibreTranslate", long_about = None)]
struct Args {
    /// The text to translate
    text: String,

    /// Source language (e.g. en)
    #[arg(short, long, default_value = "en")]
    from: String,

    /// Target language (e.g. es)
    #[arg(short, long)]
    to: String,
}

#[derive(Serialize)]
struct TranslateRequest<'a> {
    q: &'a str,
    source: &'a str,
    target: &'a str,
}

#[derive(Deserialize)]
struct TranslateResponse {
    translatedText: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = Client::new();
    let request = TranslateRequest {
        q: &args.text,
        source: &args.from,
        target: &args.to,
    };

    let response = client
        .post("http://localhost:5000/translate")
        .json(&request)
        .send()
        .await?
        .json::<TranslateResponse>()
        .await?;

    println!("{}", response.translatedText);

    Ok(())
}
