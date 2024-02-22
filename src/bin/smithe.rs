use anyhow::Result;
use smithe_lib::common::init_logger;
use std::str::FromStr;

use clap::{Parser, Subcommand};
use smithereens::smithe::{event::handle_event, player::handle_player, player::handle_id, player::handle_slug};
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

    /// Gets player related info (using id)
    Id {
        #[clap(value_parser)]
        id: i32,
    },

    /// Gets player related info (using slug)
    Slug {
        #[clap(value_parser)]
        slug: String,
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
        Commands::Id { id } => handle_id(id).await,
        Commands::Slug { slug } => handle_slug(slug).await,
    }
}
