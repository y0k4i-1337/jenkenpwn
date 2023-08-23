mod core;
mod logger;
mod utils;

use crate::core::dump::Dumper;
use clap::Parser;
use serde_json::to_string_pretty;
use tokio;
use utils::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // run the subcommand
    match &cli.subcmd {
        utils::SubCommand::Dump(args) => {
            let dumper = if let (Some(username), Some(password)) = (&args.username, &args.password)
            {
                Dumper::with_credentials(
                    args.url.clone(),
                    username.clone(),
                    password.clone(),
                    cli.verbose,
                )
            } else {
                Dumper::new(args.url.clone(), cli.verbose)
            };

            match args.resource {
                utils::DumpResource::Builds => {
                    unimplemented!("Builds are not implemented yet")
                }
                utils::DumpResource::Jobs => {
                    let result = dumper.dump_jobs().await;
                    match result {
                        Ok(_) => println!("{}", to_string_pretty(&result.unwrap())?),
                        Err(e) => println!("Error: {}", e),
                    }
                }
                utils::DumpResource::Views => {
                    unimplemented!("Views are not implemented yet")
                }
            }
        }
    }

    Ok(())
}
