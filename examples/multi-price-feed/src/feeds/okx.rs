use anyhow::{Error, Result};
use seda_sdk_rs::http_fetch;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
struct OkxData {
    last: String,
}

#[derive(Serialize, Debug, Deserialize)]
struct ApiResponse {
    data: Vec<OkxData>,
}

pub fn fetch_token_price_from_okx(symbol_a: &str, symbol_b: &str) -> Result<f32> {
    let response = http_fetch(
        format!(
            "https://www.okx.com/api/v5/market/ticker?instId={}-{}",
            symbol_a.to_uppercase(),
            symbol_b.to_uppercase()
        ),
        None,
    );

    if !response.is_ok() {
        return Err(Error::msg(format!(
            "Okx API HTTP Response was rejected: {} - ${}",
            response.status,
            String::from_utf8(response.bytes)?
        )));
    }

    let data: ApiResponse = serde_json::from_slice(&response.bytes)?;

    match data.data.first() {
        Some(info) => {
            let price: f32 = info.last.parse()?;
            Ok(price)
        }
        None => Err(Error::msg("No data returned by Okx")),
    }
}
