use serde::Deserialize;

pub mod binance;

/// Makes an HTTP request and deserializes the response
fn _make_request<T: serde::de::DeserializeOwned>(api: &str, url: String) -> anyhow::Result<T> {
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

#[derive(Deserialize)]
struct StringPriceResponse {
    price: String,
}

/// Makes a price request and converts the result to u128 with specified decimals
#[inline]
fn make_request_price(api: &str, url: String, decimals: u32) -> anyhow::Result<u128> {
    let data = _make_request::<StringPriceResponse>(api, url)?;
    let price: u128 = make_price(&data.price, decimals)?;

    Ok(price)
}

/// Converts a price string to u128 with specified decimal precision
/// Returns an error if the conversion fails or would cause overflow
fn make_price(price_str: &str, decimals: u32) -> anyhow::Result<u128> {
    // Remove decimal point and parse as integer
    let price_without_decimal = price_str.replace('.', "");

    if price_without_decimal.is_empty() {
        return Err(anyhow::anyhow!("Empty price string"));
    }

    let price_int = price_without_decimal
        .parse::<u128>()
        .map_err(|_| anyhow::anyhow!("Failed to parse price: {}", price_str))?;

    // Count decimal places in original string
    let decimal_places = price_str.split('.').nth(1).map(|s| s.len()).unwrap_or(0) as u32;

    // Adjust by the difference between desired decimals and actual decimal places
    let result = if decimal_places >= decimals {
        // Truncate extra precision
        price_int / 10_u128.pow(decimal_places - decimals)
    } else {
        // Add missing precision
        let multiplier = 10_u128.pow(decimals - decimal_places);
        price_int
            .checked_mul(multiplier)
            .ok_or_else(|| anyhow::anyhow!("Price conversion overflow"))?
    };

    Ok(result)
}
