use anyhow::Result;

const API_URL: &str = "https://api.binance.com/api/v3/ticker/price?symbol=";

/// Fetches the current price for a trading pair from Binance API
///
/// # Arguments
/// * `symbol_a` - The base symbol (e.g., "BTC")
/// * `symbol_b` - The quote symbol (e.g., "USD")
/// * `decimals` - The number of decimal places for the returned price
///
/// # Returns
/// The price as u128 with the specified decimal precision
pub fn fetch_token_price(symbol_a: &str, symbol_b: &str, decimals: u32) -> Result<u128> {
    // Validate input symbols
    if symbol_a.is_empty() || symbol_b.is_empty() {
        return Err(anyhow::anyhow!(
            "Invalid symbols: '{}' and '{}'",
            symbol_a,
            symbol_b
        ));
    }

    let url = format!("{API_URL}{symbol_a}{symbol_b}");
    crate::feeds::make_request_price("Binance", url, decimals)
}
