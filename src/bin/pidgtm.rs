use std::env;

use anyhow::Result;
use clap::{Parser, Subcommand};
use smithe_lib::common::init_logger;
use smithereens::pidgtm::{compile::handle_compile, inspect::handle_inspect, map::handle_map};

/// pidgtm stands for "player id to gamer tag mapper". This is a CLI that allows
/// direct user access to the engine that powers searching players by name.
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
    /// The StartGG API key to use for requests
    #[clap(long)]
    startgg_token: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Adds new players to the database, and updates existing players
    Map {
        #[clap(value_parser)]
        start_at_player_id: Option<i32>,
        end_at_player_id: Option<i32>,
    },
    /// Inspects a singular player from a provided playerId
    Inspect {
        #[clap(value_parser)]
        player_id: i32,
    },
    /// Compile will aggregate all player data (games, sets, tournaments, etc.), and also map.
    Compile {
        #[clap(value_parser)]
        start_at_player_id: Option<i32>,
        end_at_player_id: Option<i32>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;

    let cli = Cli::parse();

    if let Some(token) = cli.startgg_token {
        env::set_var("STARTGG_TOKEN", token);
    }

    match &cli.commands {
        Commands::Map {
            start_at_player_id,
            end_at_player_id,
        } => handle_map(*start_at_player_id, *end_at_player_id).await,
        Commands::Inspect { player_id } => handle_inspect(*player_id).await,
        Commands::Compile {
            start_at_player_id,
            end_at_player_id,
        } => handle_compile(*start_at_player_id, *end_at_player_id).await,
    }
}
