use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
struct BinancePriceFeedResponse {
    price: String,
}

const API_URL: &str = "https://api.binance.com/api/v3/ticker/price?symbol=";

pub fn fetch_token_price_from_binance(symbol_a: &str, symbol_b: &str) -> Result<f32> {
    let url = format!("{API_URL}{symbol_a}_{symbol_b}",);
    let data: BinancePriceFeedResponse = crate::feeds::make_request("Binance", url)?;
    let price: f32 = data.price.parse()?;

    Ok(price)
}
