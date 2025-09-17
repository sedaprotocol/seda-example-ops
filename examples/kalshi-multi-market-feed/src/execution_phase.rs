use anyhow::Result;
use seda_sdk_rs::{Process, elog, http_fetch, log};
use serde::{Deserialize, Serialize};

// ============================================================================
// DATA STRUCTURES
// ============================================================================

#[derive(Serialize, Deserialize)]
struct KalshiMarket {
    yes_bid: u16,
}

#[derive(Serialize, Deserialize)]
struct KalshiMarketResponse {
    market: KalshiMarket,
}

// ============================================================================
// EXECUTION PHASE - FETCHES LIVE DATA FROM KALSHI
// ============================================================================

/**
 * Executes the data request phase within the SEDA network.
 * This phase fetches yes bid prices for Kalshi markets based on comma-separated market ticker inputs.
 */
pub fn execution_phase() -> Result<()> {
    // Retrieve the input parameters for the data request (DR).
    // Expected to be comma-separated market tickers (e.g., "KXGDP-24DEC31,KXGDP-25MAR31").

    let dr_inputs_raw = String::from_utf8(Process::get_inputs())?;

    let dr_inputs_trimmed = dr_inputs_raw.trim();

    let market_tickers: Vec<&str> = dr_inputs_trimmed.split(',').collect();

    let mut yes_bids: Vec<u16> = Vec::new();

    for market_ticker in market_tickers {
        log!("Fetching Kalshi market data for market: {}", market_ticker);

        // Step 1: Get market information
        let series_response = http_fetch(
            format!(
                "https://api.elections.kalshi.com/trade-api/v2/markets/{}",
                market_ticker
            ),
            None,
        );

        // Check if the market request was successful
        if !series_response.is_ok() {
            elog!(
                "Market HTTP Response was rejected: {} - {}",
                series_response.status,
                String::from_utf8(series_response.bytes)?
            );
            Process::error("Error while fetching market information".as_bytes());
            continue;
        }

        // Parse market information
        let market_data = serde_json::from_slice::<KalshiMarketResponse>(&series_response.bytes)?;
        log!(
            "Fetched Price (YES BID): {} cents",
            market_data.market.yes_bid
        );

        yes_bids.push(market_data.market.yes_bid);
    }

    let yes_bids_bytes = serde_json::to_vec(&yes_bids)?;
    Process::success(&yes_bids_bytes);
    Ok(())
}
