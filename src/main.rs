use clap::{Parser, Subcommand};
use colored::Colorize;
use regex::Regex;
use std::fs;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "safe-push")]
#[command(about = "Blocks secret/PII leaks at commit time")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a directory or file for secrets
    Scan {
        /// Path to scan (default: current dir)
        path: Option<String>,
    },
    /// Sanitize a file (replace secrets with [REDACTED])
    Sanitize {
        /// Path to file
        path: String,
        /// Output path (defaults to stdout if not provided)
        output: Option<String>,
    },
}

/// Returns labeled patterns used for detection in scan output.
fn labeled_patterns() -> Vec<(Regex, &'static str)> {
    vec![
        (Regex::new(r"(?i)aws[_-]?access[_-]?key[_-]?id").unwrap(), "AWS Access Key ID"),
        (Regex::new(r"(?i)aws[_-]?secret[_-]?access[_-]?key").unwrap(), "AWS Secret Key"),
        (Regex::new(r"ghp_[a-zA-Z0-9]{36}").unwrap(), "GitHub Personal Token"),
        (Regex::new(r"sk_live_[a-zA-Z0-9]{24}").unwrap(), "Stripe Live Key"),
        (Regex::new(r"-----BEGIN RSA PRIVATE KEY-----").unwrap(), "RSA Private Key"),
        (
            Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap(),
            "Email Address (PII)",
        ),
    ]
}

/// Returns patterns used for replacement during sanitization.
fn sanitize_patterns() -> Vec<Regex> {
    vec![
        Regex::new(r"(?i)(aws[_-]?access[_-]?key[_-]?id|aws[_-]?secret[_-]?access[_-]?key)").unwrap(),
        Regex::new(r"ghp_[a-zA-Z0-9]{36}").unwrap(),
        Regex::new(r"sk_live_[a-zA-Z0-9]{24}").unwrap(),
        Regex::new(r"-----BEGIN RSA PRIVATE KEY-----").unwrap(),
        Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap(),
    ]
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            let target_path = path.unwrap_or_else(|| ".".to_string());
            println!("Scanning {} for secrets...", target_path);

            let mut found_issues = false;
            let patterns = labeled_patterns();

            let entries = WalkDir::new(&target_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file());

            for entry in entries {
                let path = entry.path();
                if let Ok(content) = fs::read_to_string(path) {
                    for (pattern, name) in &patterns {
                        if pattern.is_match(&content) {
                            println!("{} found in {}", name.red(), path.display());
                            found_issues = true;
                        }
                    }
                }
            }

            if found_issues {
                eprintln!("{}", "\n SafePush: Secrets detected! Commit blocked.".bright_red());
                std::process::exit(1);
            } else {
                println!("{}", "No secrets found. Safe to push!".bright_green());
            }
        }
        Commands::Sanitize { path, output } => {
            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Could not read file {}: {}", path, e);
                    std::process::exit(1);
                }
            };
            let mut sanitized = content.clone();

            for pattern in sanitize_patterns() {
                sanitized = pattern.replace_all(&sanitized, "[REDACTED]").to_string();
            }

            match output {
                Some(ref out_path) => {
                    if let Err(e) = fs::write(out_path, sanitized) {
                        eprintln!("Could not write output {}: {}", out_path, e);
                        std::process::exit(1);
                    }
                    println!("Sanitized file saved to {}", out_path);
                }
                None => {
                    println!("{}", sanitized);
                }
            }
        }
    }
}
