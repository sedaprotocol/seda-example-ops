use anyhow::Result;
#[cfg(any(feature = "testnet", feature = "mainnet"))]
use seda_sdk_rs::{Process, elog, log, proxy_http_fetch};
use serde_json::{Map, Value};

#[cfg(feature = "testnet")]
const API_1_URL: &str = "http://34.78.7.237:5384/proxy/usd/";
#[cfg(feature = "testnet")]
const PROXY_1_PUBLIC_KEY: &str =
    "02ee9686b002e8f57f9a2ca7089a6b587c9ef4e6c2b67159add5151a42ce5e6668";

#[cfg(feature = "testnet")]
const API_2_URL: &str = "https://api.hitbtc.com/api/3/public/price/ticker?symbols=";
#[cfg(feature = "testnet")]
// const PROXY_2_PUBLIC_KEY: &str = "";
#[cfg(feature = "mainnet")]
const API_1_URL: &str = "http://34.77.123.159:5384/proxy/";
#[cfg(feature = "mainnet")]
const PROXY_1_PUBLIC_KEY: &str =
    "02095af5db08cef43871a4aa48a80bdddc5249e4234e7432c3d7eca14f31261b10";

#[cfg(feature = "mainnet")]
const API_2_URL: &str = "http://:5384/proxy/usd/";
#[cfg(feature = "mainnet")]
const PROXY_2_PUBLIC_KEY: &str = "";

#[cfg(not(any(feature = "testnet", feature = "mainnet")))]
pub fn execution_phase() -> Result<()> {
    compile_error!("Either feature \"testnet\" or \"mainnet\" must be enabled");
    Ok(())
}

fn make_request<T>(url: &str, public_key: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let response = proxy_http_fetch(url, Some(public_key.to_string()), None);

    if !response.is_ok() {
        elog!(
            "HTTP Response was rejected: {} - {}",
            response.status,
            String::from_utf8(response.bytes)?
        );
        Process::error("Error while fetching symbol prices".as_bytes());
        return Err(anyhow::anyhow!("HTTP request failed"));
    }

    serde_json::from_slice(&response.bytes).map_err(|e| e.into())
}

#[cfg(any(feature = "testnet", feature = "mainnet"))]
pub fn execution_phase() -> Result<()> {
    // Retrieve the input parameters for the data request (DR).
    // Expected to be in the format "symbolA,SymbolB,..." (e.g., "BTC,ETH").

    use seda_sdk_rs::http_fetch;
    let dr_inputs_raw = String::from_utf8(Process::get_inputs())?;

    if dr_inputs_raw.is_empty() {
        // If no input is provided, log an error and return.
        elog!("No input provided for the price feed request.");
        Process::error("No input provided".as_bytes());
        return Ok(());
    }

    // Log the asset pair being fetched as part of the Execution Standard Out.
    log!("Fetching price for pair: {dr_inputs_raw}");

    let url_1 = [API_1_URL, &dr_inputs_raw].concat();
    let response_1_data = make_request::<Map<String, Value>>(&url_1, PROXY_1_PUBLIC_KEY)?;

    // Extract the prices for each symbol from the response data.
    let prices_1 = response_1_data
        .values()
        .map(|price| (price["usd"].as_f64().unwrap_or_default() * 1000000f64) as u128);

    let mut inputs = dr_inputs_raw.replace(",", "USDT,");
    inputs.push_str("USDT");
    let url_2 = [API_2_URL, &inputs].concat();
    // let response_2 = proxy_http_fetch(url_2, Some(PROXY_2_PUBLIC_KEY.to_string()), None);
    let response_2 = http_fetch(url_2, None);

    if !response_2.is_ok() {
        elog!(
            "HTTP Response was rejected: {} - {}",
            response_2.status,
            String::from_utf8(response_2.bytes)?
        );
        Process::error("Error while fetching symbol prices".as_bytes());
        return Err(anyhow::anyhow!("HTTP request failed"));
    }

    let response_2_data: Map<String, Value> = serde_json::from_slice(&response_2.bytes)?;
    let prices_2 = response_2_data
        .values()
        .map(|price| (price["price"].as_f64().unwrap_or_default() * 1000000f64) as u128);

    // average the prices from both responses
    let prices: Vec<u128> = prices_1
        .zip(prices_2)
        .map(|(p1, p2)| (p1 + p2) / 2)
        .collect();

    let result = serde_json::to_vec(&prices)?;

    // Report the successful result back to the SEDA network.
    Process::success(&result);

    Ok(())
}
