use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use serde_json::{Value, json};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("RPC error: {0}")]
    Rpc(String),

    #[error("Hex decoding error: {0}")]
    Hex(#[from] hex::FromHexError),

    #[error("Integer parsing error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
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


fn rpc_call(
    client: &Client,
    rpc_url: &str,
    method: &str,
    params: Vec<Value>,
) -> Result<Value, CliError> {
    let body = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1,
    });

    let response = client.post(rpc_url).json(&body).send()?;
    let resp_json: Value = response.json()?;
    if let Some(error) = resp_json.get("error") {
        return Err(CliError::Rpc(error.to_string()));
    }
    Ok(resp_json["result"].clone())
}

fn get_block_number(client: &Client, rpc_url: &str) -> Result<u64, CliError> {
    let result = rpc_call(client, rpc_url, "eth_blockNumber", vec![])?;
    let hex_str = result.as_str().ok_or_else(|| CliError::Rpc("result is not a string".to_string()))?.trim_start_matches("0x");
    let block_number = u64::from_str_radix(hex_str, 16)?;
    Ok(block_number)
}

fn get_balance(client: &Client, rpc_url: &str, address: &str) -> Result<f64, CliError> {
    let params = vec![json!(address), json!("latest")];
    let result = rpc_call(client, rpc_url, "eth_getBalance", params)?;
    let hex_str = result.as_str().ok_or_else(|| CliError::Rpc("balance result is not a string".to_string()))?.trim_start_matches("0x");
    let wei = u128::from_str_radix(hex_str, 16)?;
    let eth = wei as f64 / 1e18;
    Ok(eth)
}

fn main() {
    let cli = Cli::parse();
    let client = Client::new();

    match cli.command {
        Commands::BlockNumber { rpc_url } => match get_block_number(&client, &rpc_url) {
            Ok(num) => println!("Current block number: {}", num),
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::Balance { rpc_url, address } => match get_balance(&client, &rpc_url, &address) {
            Ok(balance) => println!("Balance of {}: {} ETH", address, balance),
            Err(e) => eprintln!("Error: {}", e),
        },
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_block_number() {
        
        let hex_block = "0x134567";
        let parsed = u64::from_str_radix(hex_block.trim_start_matches("0x"), 16).unwrap();
        assert_eq!(parsed, 1262951);
    }

    #[test]
    fn test_parse_balance() {
        
        let hex_wei = "0x1bc16d674ec80000";
        let wei = u128::from_str_radix(hex_wei.trim_start_matches("0x"), 16).unwrap();
        let eth = wei as f64 / 1e18;
        assert!((eth - 2.0).abs() < 1e-12);
    }
}