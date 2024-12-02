

use clap::Parser;
use tracing::{info, error, Level};

mod cli;
mod error;
mod config;
mod model;
mod rpc;

use cli::{Cli, Commands};
use error::{AnalyzerError, Result};
use config::AppConfig;
use rpc::SolanaRpcClient;

fn setup_logging() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
   
    setup_logging();

    // Load configuration
    let config = match AppConfig::new() {
        Ok(cfg) => {
            info!("Loaded configuration for: {}", cfg.app_name);
            cfg
        },
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(AnalyzerError::configuration_error("Could not load app configuration"));
        }
    };

    // Parse CLI arguments
    let cli = Cli::parse();

    // Handle optional name
    if let Some(name) = cli.name {
        info!("Operating with name: {}", name);
    }

    // Process debug level
    match cli.debug {
        0 => info!("Debug mode: Off"),
        1 => info!("Debug mode: Low"),
        2 => info!("Debug mode: Medium"),
        _ => info!("Debug mode: High"),
    }

    // Handle subcommands
    match &cli.command {
        Some(Commands::List { filter }) => {
            info!("Listing commands");
            if let Some(f) = filter {
                info!("Applying filter: {}", f);
            }
        }
        Some(Commands::Token { address }) => {
            info!("Fetching token info for address: {}", address);
            
            // Create RPC client
            let rpc_client = SolanaRpcClient::new(&config);
            
            // Fetch token supply
            match rpc_client.get_token_supply(address).await {
                Ok(token_info) => {
                    info!("Token Information:");
                    info!("Address: {}", token_info.address);
                    info!("Supply: {}", token_info.supply);
                    info!("Decimals: {}", token_info.decimals);
                }
                Err(e) => {
                    error!("Failed to fetch token info: {}", e);
                }
            }
        }
        Some(Commands::Config { verbose }) => {
            info!("Checking configuration");
            if *verbose {
                info!("App Name: {}", config.app_name);
                info!("Solana RPC: {}", config.api_endpoints.solana_rpc);
               
            }
        }
        None => {
            info!("No command specified. Use --help to see available commands.");
        }
    }

    Ok(())
}