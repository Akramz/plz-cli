#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use bat::PrettyPrinter;
use clap::Parser;
use colored::Colorize;
use config::Config;
use question::{Answer, Question};
use reqwest::blocking::Client;
use serde_json::json;
use std::process::Command;

mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

// Initialize the CLI object
struct Cli {

    /// Description of the command to execute
    prompt: Vec<String>,

}

fn main() {

    // Create the CLI object
    let cli = Cli::parse();

    // Create the config object
    let config = Config::new();

    // Create the client object
    let client = Client::new();

    // Attempt to get the user's OS
    let os_hint = if cfg!(target_os = "macos") {
        " (on macOS)"
    } else if cfg!(target_os = "linux") {
        " (on Linux)"
    } else {
        ""
    };

    // Prompt OpenAI's text-davinci-03 model and get the response
    let response = client
        .post("https://api.openai.com/v1/completions")
        .json(&json!({
            "top_p": 1,
            "stop": "```",
            "temperature": 0,
            "suffix": "\n```",
            "max_tokens": 1000,
            "presence_penalty": 0,
            "frequency_penalty": 0,
            "model": "text-davinci-003",
            "prompt": format!("{}{}:\n```bash\n#!/bin/bash\n", cli.prompt.join(" "), os_hint),
        }))
        .header("Authorization", format!("Bearer {}", config.api_key))
        .send()
        .unwrap()
        .error_for_status()
        .unwrap_or(std::process::exit(1));

    // Get the code from OpenAI's response
    let code = response.json::<serde_json::Value>().unwrap()["choices"][0]["text"]
        .as_str()
        .unwrap()
        .trim()
        .to_string();

    // Output the code
    PrettyPrinter::new()
        .input_from_bytes(code.as_bytes())
        .language("bash")
        .grid(true)
        .print()
        .unwrap();
}
