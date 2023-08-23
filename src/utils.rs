use clap::{Args, Parser, Subcommand, ValueEnum};
use regex::Regex;
use reqwest::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// verbose mode
    #[arg(short, long)]
    pub verbose: bool,
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// dump builds
    Dump(DumpArgs),
}

#[derive(Args, Debug)]
pub struct DumpArgs {
    /// username for authentication
    #[arg(short, long)]
    pub username: Option<String>,
    /// password for authentication
    #[arg(short, long)]
    pub password: Option<String>,
    /// recover from server failure, skiping already downloaded builds
    #[arg(short, long)]
    pub recover: bool,
    /// output directory
    #[arg(short, long, default_value = "dumps")]
    pub output: String,
    /// dump only the last build of each job
    #[arg(short, long)]
    pub last: bool,
    /// resources to dump
    pub resource: DumpResource,
    /// url of the jenkins server
    pub url: String,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum DumpResource {
    /// dump builds
    Builds,
    /// dump jobs
    Jobs,
    /// dump views
    Views,
}

/// Concatenate the given path to the given url
pub fn concatenate_url(
    base_url: &str,
    endpoint: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let base = Url::parse(base_url).expect("Invalid base url");
    let sanitized = base.join(endpoint).expect("Error concatenating url");

    Ok(sanitized.to_string())
}

pub fn search_substring(text: &str, search_string: &str) -> bool {
    // Enable "single line" mode using the (?s) flag
    let regex_pattern = format!("(?s){}", search_string);
    let regex = Regex::new(&regex_pattern).unwrap();
    regex.is_match(text)
}
