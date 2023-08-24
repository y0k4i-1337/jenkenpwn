mod core;
mod logger;
mod utils;

use crate::core::dump::Dumper;
use clap::Parser;
use log::warn;
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
            utils::create_directory(&args.output)?;
            match args.resource {
                utils::DumpResource::Builds => {
                    unimplemented!("Builds are not implemented yet")
                }
                utils::DumpResource::Jobs => {
                    let result = dumper.dump_jobs(args.last).await;
                    match result {
                        Ok(jobs) => {
                            let output = format!("{}/jobs.json", args.output);
                            utils::save_json(&jobs, &output)?;
                        }
                        Err(e) => {
                            warn!("Error dumping jobs: {}", e);
                        }
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
