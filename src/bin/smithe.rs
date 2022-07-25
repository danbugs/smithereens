use anyhow::Result;
use std::str::FromStr;

use clap::{Parser, Subcommand};
use smithereens::event::handle_event;
use url::Url;

/// Smithereens (or, smithe) is a digested open-source data visualizer tool for your Smash results.
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Gets player related info
    Player {
        #[clap(value_parser)]
        tag: String,
    },

    /// Gets tournament related info
    Event {
        #[clap(value_parser)]
        url: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Player { tag: _ } => todo!("no player functionality yet"),
        Commands::Event { url } => handle_event(Url::from_str(url)?).await?,
    };

    Ok(())
}
