mod core;
mod utils;

use clap::Parser;
use utils::Cli;

fn main() {
    let cli = Cli::parse();

    match &cli.subcmd {
        utils::SubCommand::Dump(args) => {
            println!("{:?}", args);
        }
    }

    println!("{:?}", cli);
}
