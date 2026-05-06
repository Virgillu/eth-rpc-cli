eth-rpc-cli

A lightweight Rust CLI tool to query Ethereum blockchain data via JSON‑RPC.
Built with reqwest, serde_json, and clap.

✨ Features

Get current block number
Get ETH balance of any address
🛠️ Build

bash
cargo build --release
The binary will be located at target/release/eth-rpc-cli.

🚀 Usage

1. Block number

bash
./target/release/eth-rpc-cli block-number --rpc-url <RPC_URL>
2. Balance

bash
./target/release/eth-rpc-cli balance --rpc-url <RPC_URL> --address <0x...>
📝 Examples

Sepolia testnet (public endpoint)

bash
# Get block number
./target/release/eth-rpc-cli block-number --rpc-url https://rpc.sepolia.org

# Get balance of an address
./target/release/eth-rpc-cli balance --rpc-url https://rpc.sepolia.org --address 0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045
Ethereum mainnet (using Alchemy)

bash
./target/release/eth-rpc-cli block-number --rpc-url https://eth-mainnet.g.alchemy.com/v2/YOUR_API_KEY
🧪 Testing

Run unit tests (if added in future) with:

bash
cargo test
📦 Dependencies

clap – command line argument parsing
reqwest – HTTP client
serde / serde_json – JSON serialization
hex – hex decoding
See Cargo.toml for full list.

🔧 Next improvements

Add tx command (look up transaction by hash)
Support ERC‑20 token balance via eth_call
Read RPC URL from .env file
Async version with tokio
WebSocket subscription for new blocks
📄 License

MIT © Virgillu
