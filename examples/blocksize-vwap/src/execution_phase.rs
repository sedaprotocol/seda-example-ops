use anyhow::{Result, anyhow};
#[cfg(any(feature = "testnet", feature = "mainnet"))]
use seda_sdk_rs::{Process, elog, log, proxy_http_fetch};

#[cfg(feature = "testnet")]
const API_URL: &str = "https://seda-proxy.blocksize.dev/proxy/vwap/";
#[cfg(feature = "testnet")]
const PROXY_PUBLIC_KEY: &str = "029a10be2771c4933b1a0f4d5efa9d6cdfbd05b0b1749587fc1b1771394490d29b";

#[cfg(feature = "mainnet")]
const API_URL: &str = "";
#[cfg(feature = "mainnet")]
const PROXY_PUBLIC_KEY: &str = "";

#[cfg(not(any(feature = "testnet", feature = "mainnet")))]
pub fn execution_phase() -> Result<()> {
    compile_error!("Either feature \"testnet\" or \"mainnet\" must be enabled");
    Ok(())
}

// {
//   "ticker": "BTCUSD",
//   "price": 112269.91858575967,
//   "size": 4.5646076099999995,
//   "volume": 512468.12475063896,
//   "ts": 1756147348689
// }

#[cfg(any(feature = "testnet", feature = "mainnet"))]
pub fn execution_phase() -> Result<()> {
    #[cfg(feature = "mainnet")]
    unimplemented!("Mainnet not yet supported");

    // Expected to be in the format "symbol" (e.g., "ETHUSD" or "BTCUSD").
    let dr_inputs_raw = String::from_utf8(Process::get_inputs())?;
    if dr_inputs_raw.is_empty() {
        // If no input is provided, log an error and return.
        elog!("No input provided for the price pair request.");
        Process::error("No input provided".as_bytes());
        return Ok(());
    }

    // Log the asset being fetched as part of the Execution Standard Out.
    log!("Fetching price for: {dr_inputs_raw}");

    let url = [API_URL, &dr_inputs_raw].concat();
    log!("Fetching URL: {url}");
    let response = proxy_http_fetch(url, Some(PROXY_PUBLIC_KEY.to_string()), None);

    // Handle the case where the HTTP request failed or was rejected.
    if !response.is_ok() {
        elog!(
            "HTTP Response was rejected: {} - {} ProxyPublicKey {PROXY_PUBLIC_KEY}",
            response.status,
            String::from_utf8(response.bytes)?
        );
        Process::error("Error while fetching price pair".as_bytes());
        return Ok(());
    }

    // Parse the API response as defined earlier.
    let response_data = serde_json::from_slice::<serde_json::Map<String, serde_json::value::Value>>(
        &response.bytes,
    )?;

    let price = response_data
        .get("price")
        .and_then(|price| price.as_f64())
        .ok_or_else(|| anyhow::anyhow!("price not found in response or invalid"))?;
    let price_lossless = make_price(&price.to_string(), 6)?;
    log!("Fetched price: {price_lossless:?}");

    // Report the successful result back to the SEDA network.
    Process::success(&price_lossless.to_le_bytes());

    Ok(())
}

/// Convert a decimal price string (e.g., "1234.5678") into a u128 with `decimals` precision.
/// - Truncates extra precision if the input has more decimals than requested.
/// - Multiplies (with overflow checks) if the input has fewer decimals than requested.
fn make_price(price_str: &str, decimals: u32) -> Result<u128> {
    let (int_part, frac_part) = match price_str.split_once('.') {
        Some((i, f)) => (i, f),
        None => (price_str, ""),
    };

    let digits = format!("{int_part}{frac_part}");
    if digits.is_empty() {
        return Err(anyhow!("Empty price string"));
    }

    let price_int = digits
        .parse::<u128>()
        .map_err(|_| anyhow!("Failed to parse price: {price_str}"))?;

    let decimal_places = frac_part.len() as u32;

    let result = if decimal_places >= decimals {
        price_int / 10_u128.pow(decimal_places - decimals)
    } else {
        let mul = 10_u128
            .checked_pow(decimals - decimal_places)
            .ok_or_else(|| anyhow!("Price conversion overflow"))?;
        price_int
            .checked_mul(mul)
            .ok_or_else(|| anyhow!("Price conversion overflow"))?
    };

    Ok(result)
}
