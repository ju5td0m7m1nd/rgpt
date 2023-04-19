use std::error::Error;

use clap::{App, Arg};
use reqwest::{header, Client};
use serde_json::{from_str, json, Map, Value};

mod api_key;
mod settings;

// Constants for the OpenAI API endpoint and authentication
const API_URL: &str = "https://api.openai.com/v1/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let openai_api_key = api_key::API_KEY;

    // Define command-line arguments
    let matches = App::new("rgpt")
        .version("0.1")
        .author("MH Tsai (@mhtsai_95)")
        .about("A CLI tool for interacting with the OpenAI API with your preset commands.")
        .arg(Arg::with_name("command").required(true).takes_value(true))
        .arg(Arg::with_name("text").required(true).takes_value(true))
        .get_matches();

    let settings: Map<String, Value> = from_str(settings::SETTINGS)?;

    // Get prompt for specified command
    let command = matches.value_of("command").unwrap();
    let prompt = settings
        .get(command)
        .ok_or_else(|| "Command not found")?
        .get("prompt")
        .unwrap_or(&Value::Null)
        .to_string();

    let text = matches.value_of("text").unwrap();

    // Send request to OpenAI API
    let client = Client::new();
    let response = client
        .post(&format!("{}chat/completions", API_URL))
        .header(header::AUTHORIZATION, format!("Bearer {}", openai_api_key))
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "user",
                    "content": format!("{} {}", prompt, text),
                  },
            ],
            "temperature": 0.7,
            "stop": ["\n"]
        }))
        .send()
        .await?;
    let response_body = response.json::<serde_json::Value>().await?;
    let result = response_body["choices"][0]["message"]
        .get("content")
        .unwrap_or(&Value::Null);

    // let mut corrected_text = String::new();
    // for (i, line) in result.lines().enumerate() {
    //     if i > 0 {
    //         corrected_text.push('\n');
    //     }
    //     corrected_text.push_str(line.trim_start());
    // }

    println!("{}", result);

    Ok(())
}
