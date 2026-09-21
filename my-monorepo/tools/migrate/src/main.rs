use clap::{Parser, Subcommand};
use data_model::NewUser;
use log::info;

#[derive(Parser, Debug)]
#[command(name = "migrate", version, about = "Monorepo tool example")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show migration status
    Status,
    /// Apply migrations up to latest
    Apply,
    /// Revert last migration
    Revert,
    /// Quick demo using shared types
    DemoUser { name: String },
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Status => info!("status: up-to-date (demo)"),
        Commands::Apply => info!("apply: nothing to do (demo)"),
        Commands::Revert => info!("revert: nothing to revert (demo)"),
        Commands::DemoUser { name } => {
            let new = NewUser { name };
            match new.validate() {
                Ok(()) => info!("new user payload is valid"),
                Err(e) => info!("invalid payload: {e}"),
            }
        }
    }

    Ok(())
}
