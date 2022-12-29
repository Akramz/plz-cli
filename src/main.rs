#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use bat::PrettyPrinter;
use clap::Parser;
use colored::Colorize;
use config::Config;
use question::{Answer, Question};
use reqwest::blocking::Client;
use serde_json::json;
use spinners::{Spinner, Spinners};
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

    // Create the spinner used to inform the user to wait
    let mut spinner = Spinner::new(Spinners::BouncingBar, "Generating your command...".into());

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
        .unwrap_or_else(|_| {
            spinner.stop_and_persist(
                "✖".red().to_string().as_str(),
                "Failed to get a response. Have you set the OPENAI_API_KEY variable?"
                    .red()
                    .to_string(),
            );
            std::process::exit(1);
        });

    // Get the code from OpenAI's response
    let code = response.json::<serde_json::Value>().unwrap()["choices"][0]["text"]
        .as_str()
        .unwrap()
        .trim()
        .to_string();

    // Stop the spinner and output the code
    spinner.stop_and_persist(
        "✔".green().to_string().as_str(),
        "Got some code!".green().to_string(),
    );
    PrettyPrinter::new()
        .input_from_bytes(code.as_bytes())
        .language("bash")
        .grid(true)
        .print()
        .unwrap();
}
