use anyhow::Result;
#[cfg(any(feature = "testnet", feature = "mainnet"))]
use seda_sdk_rs::{Process, elog, log, proxy_http_fetch};

#[cfg(feature = "testnet")]
const API_URL: &str = "http://98.84.79.123:5384/proxy/cfd/";
#[cfg(feature = "testnet")]
const PROXY_PUBLIC_KEY: &str = "0375038bc3e61dc2a52e24ff207a5753e38d020a06fff9efc8ec96875f72f4d081";

#[cfg(feature = "mainnet")]
const API_URL: &str = "http://:5384/proxy/";
#[cfg(feature = "mainnet")]
const PROXY_PUBLIC_KEY: &str = "";

#[cfg(not(any(feature = "testnet", feature = "mainnet")))]
pub fn execution_phase() -> Result<()> {
    compile_error!("Either feature \"testnet\" or \"mainnet\" must be enabled");
    Ok(())
}

// {
//   "Quote": {
//     "XAU/USD:BFX": {
//       "askExchangeCode": "",
//       "askPrice": 3313.99,
//       "askSize": 100,
//       "askTime": 1753710744000,
//       "bidExchangeCode": "",
//       "bidPrice": 3313.83,
//       "bidSize": 100,
//       "bidTime": 1753710744000,
//       "eventSymbol": "XAU/USD:BFX",
//       "eventTime": 0,
//       "sequence": 0,
//       "timeNanoPart": 0
//     }
//   },
//   "status": "OK"
// }

/// The response structure for commodity price requests.
#[derive(serde::Deserialize)]
struct CommodityPriceResponse {
    #[serde(rename = "Quote")]
    quote: serde_json::value::Map<String, serde_json::value::Value>,
}

#[cfg(any(feature = "testnet", feature = "mainnet"))]
pub fn execution_phase() -> Result<()> {
    #[cfg(feature = "mainnet")]
    unimplemented!("Mainnet execution phase is not yet implemented");

    // Expected to be in the format "symbol,..." (e.g., "XAU" or "BRN").
    let dr_inputs_raw = String::from_utf8(Process::get_inputs())?;

    // If no input is provided, log an error and return.
    if dr_inputs_raw.is_empty() {
        elog!("No input provided for the commodity price request.");
        Process::error("No input provided".as_bytes());
        return Ok(());
    }

    // Log the asset being fetched as part of the Execution Standard Out.
    log!("Fetching price for: {dr_inputs_raw}");

    // Get the price in USD
    let url = [API_URL, &dr_inputs_raw, "/USD"].concat();
    let response = proxy_http_fetch(url, Some(PROXY_PUBLIC_KEY.to_string()), None);

    // Handle the case where the HTTP request failed or was rejected.
    if !response.is_ok() {
        elog!(
            "HTTP Response was rejected: {} - {}",
            response.status,
            String::from_utf8(response.bytes)?
        );
        Process::error("Error while fetching commodity price".as_bytes());
        return Ok(());
    }

    // Parse the API response as defined earlier.
    let response_data = serde_json::from_slice::<CommodityPriceResponse>(&response.bytes)?;

    let price = response_data
        .quote
        .get(&format!("{dr_inputs_raw}/USD:BFX"))
        .and_then(|quote| quote.get("askPrice"))
        .and_then(|price| price.as_f64())
        .ok_or_else(|| anyhow::anyhow!("Price not found in response"))?;
    let price_lossless = (price * 100.0) as u128;
    log!("Fetched price: {price_lossless:?}");

    // Report the successful result back to the SEDA network.
    Process::success(&price_lossless.to_le_bytes());

    Ok(())
}
