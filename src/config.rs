use colored::Colorize;
use std::{env, io::Write, process::exit};

pub struct Config {
    pub api_key: String,
    pub shell: String,
}

impl Config {
    pub fn new() -> Self {
        let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
            println!("{}", "This program requires an OpenAI API key to run. Please set the OPENAI_API_KEY environment variable. https://github.com/akramz/plz-cli#usage".red());
            exit(1);
        });

        let shell = env::var("SHELL").unwrap_or_else(|_| String::new());

        Self { api_key, shell }
    }

    pub fn write_to_history(&self, code: &str) {
        let history_file = match self.shell.as_str() {
            "/bin/bash" => std::env::var("HOME").unwrap() + "/.bash_history",
            "/bin/zsh" => std::env::var("HOME").unwrap() + "/.zsh_history",
            _ => return,
        };

        std::fs::OpenOptions::new()
            .append(true)
            .open(history_file)
            .map_or((), |mut file| {
                file.write_all(format!("{code}\n").as_bytes())
                    .unwrap_or_else(|_| {
                        exit(1);
                    });
            });
    }
}
