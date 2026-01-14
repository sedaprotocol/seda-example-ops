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
 * This phase fetches bid and ask prices for Kalshi markets based on a market ticker input.
 */
pub fn execution_phase() -> Result<()> {
    // Retrieve the input parameters for the data request (DR).
    // Expected to be a market ticker (e.g., "KXGDP").
    let dr_inputs_raw = String::from_utf8(Process::get_inputs())?;
    let market_ticker = dr_inputs_raw.trim();

    log!("Fetching Kalshi market data for market: {}", market_ticker);

    // Step 1: Get market information
    let market_response = http_fetch(
        format!(
            "https://api.elections.kalshi.com/trade-api/v2/markets/{}",
            market_ticker
        ),
        None,
    );

    // Check if the market request was successful
    if !market_response.is_ok() {
        elog!(
            "market HTTP Response was rejected: {} - {}",
            market_response.status,
            String::from_utf8(market_response.bytes)?
        );
        Process::error("Error while fetching market information".as_bytes());
        return Ok(());
    }

    // Parse market information
    let market_data = serde_json::from_slice::<KalshiMarketResponse>(&market_response.bytes)?;
    log!(
        "Fetched Price (YES BID): {} cents",
        market_data.market.yes_bid
    );

    Process::success(market_data.market.yes_bid.to_string().as_bytes());
    Ok(())
}
