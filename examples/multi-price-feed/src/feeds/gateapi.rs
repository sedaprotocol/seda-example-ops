use anyhow::{Error, Result};
use seda_sdk_rs::http_fetch;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
struct ApiResponse {
    last: String,
}

pub fn fetch_token_price_from_gateapi(symbol_a: &str, symbol_b: &str) -> Result<f32> {
    let response = http_fetch(
        format!(
            "https://data.gateapi.io/api2/1/ticker/{}_{}",
            symbol_a.to_lowercase(),
            symbol_b.to_lowercase()
        ),
        None,
    );

    if !response.is_ok() {
        return Err(Error::msg(format!(
            "GateAPI API HTTP Response was rejected: {} - ${}",
            response.status,
            String::from_utf8(response.bytes)?
        )));
    }

    let data: ApiResponse = serde_json::from_slice(&response.bytes)?;
    let price: f32 = data.last.parse()?;

    Ok(price)
}
