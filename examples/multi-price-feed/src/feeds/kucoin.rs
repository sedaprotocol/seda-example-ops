use anyhow::{Error, Result};
use seda_sdk_rs::http_fetch;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
struct KucoinData {
    price: String,
}

#[derive(Serialize, Debug, Deserialize)]
struct ApiResponse {
    data: KucoinData,
}

pub fn fetch_token_price_from_kucoin(symbol_a: &str, symbol_b: &str) -> Result<f32> {
    // use seda_sdk_rs::proxy_http_fetch;
    // let response = proxy_http_fetch(
    //     format!(
    //         "http://35.214.89.94:3000/proxy/{}/{}",
    //         symbol_a.to_uppercase(),
    //         symbol_b.to_uppercase()
    //     ),
    //     None,
    //     None,
    // );

    let response = http_fetch(
        format!(
            "https://api.kucoin.com/api/v1/market/orderbook/level1?symbol={}-{}",
            symbol_a.to_uppercase(),
            symbol_b.to_uppercase()
        ),
        None,
    );

    if !response.is_ok() {
        return Err(Error::msg(format!(
            "Kucoin API HTTP Response was rejected: {} - ${}",
            response.status,
            String::from_utf8(response.bytes)?
        )));
    }

    let data: ApiResponse = serde_json::from_slice(&response.bytes)?;
    let price: f32 = data.data.price.parse()?;

    Ok(price)
}
