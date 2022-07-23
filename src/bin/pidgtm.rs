use anyhow::Result;
use clap::{Parser, Subcommand};
use simple_smash_stats::pidgtm_command_handlers::{inspect::handle_inspect, map::handle_map};
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
    /// Starts mapping playerIds, gamerTags, and userSlugs onto the pidgtm DB
    Map,
    /// Inspects a singular player from a provided playerId
    Inspect {
        #[clap(value_parser)]
        player_id: i32,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();
    match &cli.commands {
        Commands::Map => handle_map(),
        Commands::Inspect { player_id } => handle_inspect(*player_id).await,
    }
}
