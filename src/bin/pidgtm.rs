use anyhow::Result;
use clap::{Parser, Subcommand};
use smithereens::pidgtm::{inspect::handle_inspect, map::handle_map, update::handle_update, compile::handle_compile};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

/// pidgtm stands for "player id to gamer tag mapper". This is a CLI that allows
/// direct user access to the engine that powers searching players by name.
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Starts mapping playerIds, gamerTags, userSlugs, and more onto the pidgtm DB
    Map,
    /// Inspects a singular player from a provided playerId
    Inspect {
        #[clap(value_parser)]
        player_id: i32,
    },
    /// Updates current mapping w/ new information onto the pidgtm DB
    Update {
        #[clap(value_parser)]
        start_at_player_id: Option<i32>,
    },
    /// Compile all player data from 1000-X onto the pidgtm DB
    Compile {
        #[clap(value_parser)]
        start_at_player_id: Option<i32>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();
    match &cli.commands {
        Commands::Map => handle_map().await,
        Commands::Inspect { player_id } => handle_inspect(*player_id).await,
        Commands::Update { start_at_player_id } => handle_update(*start_at_player_id).await,
        Commands::Compile { start_at_player_id } => handle_compile(*start_at_player_id).await,
    }
}
