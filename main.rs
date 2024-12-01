use clap::{Parser, Subcommand};
use crate::commands::{list_pods, list_deployments};

mod commands;

#[derive(Parser)]
#[command(name = "ktp", version = "1.0", about = "Kubernetes CLI tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all pods
    ListPods,
    /// List all deployments
    ListDeployments,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ListPods => list_pods().await?,
        Commands::ListDeployments => list_deployments().await?,
    }

    Ok(())
}