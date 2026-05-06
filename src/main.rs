use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use serde_json::{json, Value};

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

/// 发送 JSON-RPC 请求并返回结果字段
fn rpc_call(client: &Client, rpc_url: &str, method: &str, params: Vec<Value>) -> Result<Value, Box<dyn std::error::Error>> {
    let body = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1,
    });

    let response = client.post(rpc_url)
        .json(&body)
        .send()?;
    
    let resp_json: Value = response.json()?;
    if let Some(error) = resp_json.get("error") {
        return Err(format!("RPC error: {}", error).into());
    }
    Ok(resp_json["result"].clone())
}

fn get_block_number(client: &Client, rpc_url: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let result = rpc_call(client, rpc_url, "eth_blockNumber", vec![])?;
    let hex_str = result.as_str().unwrap().trim_start_matches("0x");
    let block_number = u64::from_str_radix(hex_str, 16)?;
    Ok(block_number)
}

fn get_balance(client: &Client, rpc_url: &str, address: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let params = vec![json!(address), json!("latest")];
    let result = rpc_call(client, rpc_url, "eth_getBalance", params)?;
    let hex_str = result.as_str().unwrap().trim_start_matches("0x");
    let wei = u128::from_str_radix(hex_str, 16)?;
    let eth = wei as f64 / 1e18;
    Ok(eth)
}

fn main() {
    let cli = Cli::parse();
    let client = Client::new();

    match cli.command {
        Commands::BlockNumber { rpc_url } => {
            match get_block_number(&client, &rpc_url) {
                Ok(num) => println!("Current block number: {}", num),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Balance { rpc_url, address } => {
            match get_balance(&client, &rpc_url, &address) {
                Ok(balance) => println!("Balance of {}: {} ETH", address, balance),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}