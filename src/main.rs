use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use thiserror::Error;

mod commands;
use commands::{get_balance, get_block_number};

#[derive(Error, Debug)]
pub enum CliError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("RPC error: {0}")]
    Rpc(String),
    #[error("Invalid hex string: {0}")]
    Hex(#[from] hex::FromHexError),
    #[error("Integer parsing error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

#[derive(Parser)]
#[command(name = "eth-rpc-cli")]
#[command(about = "CLI tool to query Ethereum blockchain via RPC", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get the latest block number
    BlockNumber {
        /// RPC URL (e.g., https://eth-mainnet.g.alchemy.com/v2/your_key)
        #[arg(short, long)]
        rpc_url: String,
    },
    /// Get ETH balance of an address
    Balance {
        /// RPC URL
        #[arg(short, long)]
        rpc_url: String,
        /// Ethereum address (0x...)
        #[arg(short, long)]
        address: String,
    },
}

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();
    let client = Client::new();

    match cli.command {
        Commands::BlockNumber { rpc_url } => {
            let num = get_block_number(&client, &rpc_url)?;
            println!("Current block number: {}", num);
        }
        Commands::Balance { rpc_url, address } => {
            let balance = get_balance(&client, &rpc_url, &address)?;
            println!("Balance of {}: {} ETH", address, balance);
        }
    }
    Ok(())
}
