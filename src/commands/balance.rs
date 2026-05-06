use crate::CliError;
use reqwest::blocking::Client;
use serde_json::{Value, json};

fn rpc_call(client: &Client, rpc_url: &str, method: &str, params: Vec<Value>) -> Result<Value, CliError> {
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

pub fn get_balance(client: &Client, rpc_url: &str, address: &str) -> Result<f64, CliError> {
    let params = vec![json!(address), json!("latest")];
    let result = rpc_call(client, rpc_url, "eth_getBalance", params)?;
    let hex_str = result.as_str().unwrap().trim_start_matches("0x");
    let wei = u128::from_str_radix(hex_str, 16)?;
    Ok(wei as f64 / 1e18)
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_balance() {
        let hex_wei = "0x1bc16d674ec80000";
        let wei = u128::from_str_radix(hex_wei.trim_start_matches("0x"), 16).unwrap();
        let eth = wei as f64 / 1e18;
        assert!((eth - 2.0).abs() < 1e-12);
    }
}
