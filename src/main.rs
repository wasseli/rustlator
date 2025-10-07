use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "rl")]
#[command(about = "Translate words between languages using LibreTranslate", long_about = None)]
struct Args {
    /// The text to translate
    text: Option<String>,

    /// Set the target language (-t, --to)
    #[arg(short = 't', long = "to")]
    to: Option<String>,

    /// Set the source language (-f, --from)
    #[arg(short = 'f', long = "from")]
    from: Option<String>,

    /// Show current language settings (-s, --status)
    #[arg(short, long)]
    status: bool,
}

#[derive(Serialize)]
struct TranslateRequest<'a> {
    q: &'a str,
    source: &'a str,
    target: &'a str,
}

#[derive(Deserialize)]
struct TranslateResponse {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Load config file from ~/.rustlator/config.json
    let home_dir = std::env::var("HOME")?;
    let config_path = format!("{}/.rustlator/config.json", home_dir);
    let config_contents = std::fs::read_to_string(&config_path)?;
    let mut config: serde_json::Value = serde_json::from_str(&config_contents)?;
    let api_url = config["api_url"]
        .as_str()
        .ok_or("Missing 'api_url' in configuration file")?;

    // Handle --to/--from for setting language
    if args.to.is_some() || args.from.is_some() {
        if let Some(to) = args.to {
            config["to"] = serde_json::Value::String(to);
        }
        if let Some(from) = args.from {
            config["from"] = serde_json::Value::String(from);
        }
        std::fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;
        println!("Language settings updated.");
        return Ok(());
    }

    // Determine language settings
    let from_lang = config["from"].as_str().unwrap_or("en").to_string();
    let to_lang = config["to"].as_str().unwrap_or("fi").to_string();

    // Handle status flag
    if args.status {
        println!("Current language settings:");
        println!("From: {}", from_lang);
        println!("To: {}", to_lang);
        return Ok(());
    }

    // Ensure text exists before translation
    let text = match args.text {
        Some(ref t) => t,
        None => {
            eprintln!("Error: missing TEXT argument. Either provide text to translate or use --status.");
            std::process::exit(1);
        }
    };

    let client = Client::new();

    let request = TranslateRequest {
        q: text,
        source: &from_lang,
        target: &to_lang,
    };

    let response = client
        .post(api_url)
        .json(&request)
        .send()
        .await?
        .json::<TranslateResponse>()
        .await?;

    println!("{}", response.translated_text);

    Ok(())
}
