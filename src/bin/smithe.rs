use anyhow::Result;
use smithe_lib::common::init_logger;
use std::str::FromStr;

use clap::{Parser, Subcommand};
use smithereens::smithe::{event::handle_event, player::handle_player};
use url::Url;

/// Smithereens, or Smithe, is a digested open-source data visualizer tool for your Smash results.
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
    init_logger()?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Player { tag } => handle_player(tag).await,
        Commands::Event { url } => handle_event(Url::from_str(url)?).await,
    }
}
