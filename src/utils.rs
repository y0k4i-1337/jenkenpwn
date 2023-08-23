use clap::{Parser, Subcommand, Args};

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
        username: Option<String>,
        /// password for authentication
        #[arg(short, long)]
        password: Option<String>,
        /// recover from server failure, skiping already downloaded builds
        #[arg(short, long)]
        recover: bool,
        /// output directory
        #[arg(short, long, default_value = "dumps")]
        output: String,
        /// dump only the last build of each job
        #[arg(short, long)]
        last: bool,
        /// url of the jenkins server
        url: String,
    }
