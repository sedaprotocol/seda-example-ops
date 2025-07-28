pub mod binance;
pub mod gateapi;
pub mod kucoin;
pub mod mexc;
pub mod okx;

fn make_request<T: serde::de::DeserializeOwned>(api: &str, url: String) -> anyhow::Result<T> {
    let response = seda_sdk_rs::http_fetch(url, None);

    if !response.is_ok() {
        return Err(anyhow::anyhow!(
            "{api} API HTTP Response was rejected: {} - {}",
            response.status,
            String::from_utf8(response.bytes)?
        ));
    }

    Ok(serde_json::from_slice(&response.bytes)?)
}
