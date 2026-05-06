use crate::CliError;
use reqwest::blocking::Client;
use serde_json::Value;

fn rpc_call(client: &Client, rpc_url: &str, method: &str, params: Vec<Value>) -> Result<Value, CliError> {
    let body = serde_json::json!({
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

pub fn get_block_number(client: &Client, rpc_url: &str) -> Result<u64, CliError> {
    let result = rpc_call(client, rpc_url, "eth_blockNumber", vec![])?;
    let hex_str = result.as_str().unwrap().trim_start_matches("0x");
    let block_number = u64::from_str_radix(hex_str, 16)?;
    Ok(block_number)
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_block_number() {
        let hex_block = "0x134567";
        let parsed = u64::from_str_radix(hex_block.trim_start_matches("0x"), 16).unwrap();
        assert_eq!(parsed, 1262951);
    }
}