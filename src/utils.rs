use clap::{Args, Parser, Subcommand, ValueEnum};
use log::debug;
use regex::Regex;
use reqwest::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// verbose mode
    #[arg(short, long)]
    pub verbose: bool,
    /// do not verify SSL certificate
    #[arg(short, long)]
    pub insecure: bool,
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
    /// read jobs from a jobs dump file
    #[arg(short, long)]
    pub jobs: Option<String>,
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

/// Concatenate the given path to the given url. The path can be absolute or relative.
pub fn concatenate_url(
    base_url: &str,
    endpoint: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let base = Url::parse(base_url).expect("Invalid base url");
    let sanitized = base.join(endpoint)?;
    Ok(sanitized.to_string())
}

/// Search for a substring in a text
pub fn search_substring(text: &str, search_string: &str) -> bool {
    // Enable "single line" mode using the (?s) flag
    let regex_pattern = format!("(?s){}", search_string);
    let regex = Regex::new(&regex_pattern).unwrap();
    regex.is_match(text)
}

/// Create directory if it does not exist
pub fn create_directory(directory: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new(directory).exists() {
        debug!("Creating directory: {}", directory);
        std::fs::create_dir_all(directory)?;
    }
    Ok(())
}

/// Extract path from url (eg. "http://localhost:8080/job/MyJob/1" ->
/// "job/MyJob/1")
pub fn extract_path(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parsed_url = Url::parse(url)?;
    let path = parsed_url.path().to_string();
    Ok(path)
}

/// Save JSON to file
pub fn save_json(
    json: &serde_json::Value,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Saving JSON to file: {}", filename);
    let file = std::fs::File::create(filename)?;
    serde_json::to_writer_pretty(file, json)?;
    Ok(())
}

/// Load JSON from file
pub fn load_json(filename: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    debug!("Loading JSON from file: {}", filename);
    let file = std::fs::File::open(filename)?;
    let json: serde_json::Value = serde_json::from_reader(file)?;
    Ok(json)
}
