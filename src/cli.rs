
use clap::{Parser, Subcommand};

// Solana Token Analyzer CLI
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    // Optional name to operate on
    #[arg(short, long)]
    pub name: Option<String>,

    // Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    // Subcommands for different operations
    #[command(subcommand)]
    pub command: Option<Commands>,
}

// Supported CLI commands
#[derive(Subcommand)]
pub enum Commands {
    /// List available commands
    List {
        // Optional filter for listing
        #[arg(short, long)]
        filter: Option<String>,
    },
    // Fetch token information
    Token {
        // Token address to query
        #[arg(short, long)]
        address: String,
    },
    // Check system configuration
    Config {
        // Show detailed configuration
        #[arg(short, long)]
        verbose: bool,
    },
}
