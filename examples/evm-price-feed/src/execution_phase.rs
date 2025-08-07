use anyhow::Result;
use seda_sdk_rs::{Process, elog, log};

pub fn execution_phase() -> Result<()> {
    // Retrieve the input parameters for the data request (DR).
    // Expected to be in the format "symbolA-symbolB,SymbolC-symboldD,..." (e.g., "BTC-USD,ETH-USD").
    let dr_inputs = ethabi::decode(
        &[ethabi::ParamType::Array(Box::new(
            ethabi::ParamType::String,
        ))],
        &Process::get_inputs(),
    );

    if dr_inputs.is_err() || dr_inputs.as_ref().unwrap().is_empty() {
        elog!("Invalid or missing input for price feed request.");
        Process::error("Invalid input: missing or failed to decode".as_bytes());
        return Ok(());
    }

    // Extract the inner array from the decoded result
    let dr_inputs = dr_inputs.unwrap();
    let dr_inputs = match &dr_inputs[0] {
        ethabi::Token::Array(tokens) => tokens,
        _ => {
            elog!("Expected array of strings");
            Process::error("Invalid input format".as_bytes());
            return Ok(());
        }
    };

    // Parse multiple symbol pairs
    let mut price_pairs: Vec<(String, String)> = Vec::with_capacity(dr_inputs.len());
    for pair in dr_inputs {
        let pair_str = match pair {
            ethabi::Token::String(s) => s.clone(),
            _ => {
                elog!("Expected string token, got: {:?}", pair);
                Process::error("Invalid token type".as_bytes());
                return Ok(());
            }
        };

        let parts: Vec<&str> = pair_str.split('-').collect();
        if parts.len() == 2 {
            price_pairs.push((parts[0].to_string(), parts[1].to_string()));
        } else {
            elog!("Invalid symbol pair format: {pair_str}. Expected format: symbolA-symbolB");
            Process::error(format!("Invalid symbol pair format: {pair_str}").as_bytes());
            return Ok(());
        }
    }

    let mut prices = Vec::with_capacity(price_pairs.len());
    for (base_symbol, quote_symbol) in &price_pairs {
        match crate::feeds::binance::fetch_token_price(base_symbol, quote_symbol, 6) {
            Ok(price) => {
                log!("Got price for {}-{}: {}", base_symbol, quote_symbol, price);
                prices.push(price);
            }
            Err(error) => {
                elog!(
                    "Failed to fetch price for {}-{}: {}",
                    base_symbol,
                    quote_symbol,
                    error
                );
                Process::error("Failed to fetch prices".as_bytes());
                return Ok(());
            }
        }
    }

    log!("Successfully fetched {} prices: {:?}", prices.len(), prices);
    let result = serde_json::to_vec(&prices)?;
    Process::success(&result);

    Ok(())
}
