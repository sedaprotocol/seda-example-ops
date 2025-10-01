use anyhow::Result;
#[cfg(any(feature = "testnet", feature = "mainnet"))]
use seda_sdk_rs::{Process, elog, log, proxy_http_fetch};

#[cfg(feature = "testnet")]
const API_URL: &str = "http://18.130.226.194:5384/proxy/";
#[cfg(feature = "testnet")]
const PROXY_PUBLIC_KEY: &str = "03c4dcfca973ece4da6c53d0343132a9e8801e4062017d6ae6afd09d6576a6c942";

#[cfg(feature = "mainnet")]
const API_URL: &str = todo!("http://:5384/proxy/");
#[cfg(feature = "mainnet")]
const PROXY_PUBLIC_KEY: &str = todo!("");

#[cfg(not(any(feature = "testnet", feature = "mainnet")))]
pub fn execution_phase() -> Result<()> {
    compile_error!("Either feature \"testnet\" or \"mainnet\" must be enabled");
    Ok(())
}

// {
// 	"date": "2022-09-10",
// 	"price": 15.9,
// 	"estimatedValuation": 2150000000,
// 	"priceStandardError": 1.35,
// 	"generatedAtTimestamp": 1690000000,
// 	"daysSinceLastDataPoint": 10,
// 	"numberOfPoints6mo": 12,
// 	"orderImbalance": {
// 		"bidContribution": 0.53,
// 		"offerContribution": 0.25
// 	}
// }

#[cfg(any(feature = "testnet", feature = "mainnet"))]
pub fn execution_phase() -> Result<()> {
    use seda_sdk_rs::{HttpFetchMethod, HttpFetchOptions};

    #[cfg(feature = "mainnet")]
    unimplemented!("Mainnet execution phase is not yet implemented");

    // Expected to be a valid project Id for the Caplight API.
    let dr_inputs_raw = String::from_utf8(Process::get_inputs())?;

    // If no input is provided, log an error and return.
    if dr_inputs_raw.is_empty() {
        elog!("No input provided for the equity price request.");
        Process::error("No input provided".as_bytes());
        return Ok(());
    }

    // Log the asset being fetched as part of the Execution Standard Out.
    log!("Fetching price for: {dr_inputs_raw}");

    let url = [API_URL, &dr_inputs_raw].concat();
    let response = proxy_http_fetch(
        url,
        Some(PROXY_PUBLIC_KEY.to_string()),
        Some(HttpFetchOptions {
            method: HttpFetchMethod::Get,
            headers: Default::default(),
            body: None,
            timeout_ms: Some(20_000),
        }),
    );
    log!(
        "HTTP Response: {}",
        String::from_utf8(response.bytes.clone())?
    );

    // Check if the HTTP request was successfully fulfilled or not.
    if !response.is_ok() {
        elog!(
            "HTTP Response was rejected: {} - {}",
            response.status,
            String::from_utf8(response.bytes)?
        );
        Process::error("Error while fetching equity price".as_bytes());
        return Ok(());
    }

    // Parse the API response as defined earlier.
    let response_data = serde_json::from_slice::<serde_json::Value>(&response.bytes)?;

    let price = response_data
        .get("price")
        .and_then(|price| price.as_f64())
        .ok_or_else(|| anyhow::anyhow!("Price not found in response"))?;
    let price_lossless = (price * 1_000_000.0) as u128;
    log!("Fetched price: {price_lossless:?}");

    // Report the successful result back to the SEDA network.
    Process::success(&price_lossless.to_le_bytes());

    Ok(())
}
