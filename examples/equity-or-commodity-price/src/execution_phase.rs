use anyhow::Result;
#[cfg(any(feature = "testnet", feature = "mainnet"))]
use seda_sdk_rs::{Process, elog, log, proxy_http_fetch};

#[cfg(feature = "testnet")]
const API_URL: &str = "http://98.84.79.123:5384/proxy/";
#[cfg(feature = "testnet")]
const PROXY_PUBLIC_KEY: &str = "0375038bc3e61dc2a52e24ff207a5753e38d020a06fff9efc8ec96875f72f4d081";

#[cfg(feature = "mainnet")]
const API_URL: &str = "http://seda-proxy.dxfeed.com:5384/proxy/";
#[cfg(feature = "mainnet")]
const PROXY_PUBLIC_KEY: &str = "021dd035f760061e2833581d4ab50440a355db0ac98e489bf63a5dbc0e89e4af79";

#[cfg(not(any(feature = "testnet", feature = "mainnet")))]
pub fn execution_phase() -> Result<()> {
    compile_error!("Either feature \"testnet\" or \"mainnet\" must be enabled");
    Ok(())
}

// Commodity /cfd/
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

// Equity /uslf_q/
// {
// 	"Quote": {
// 		"AAPL:USLF24": {
// 			"askExchangeCode": "U",
// 			"askPrice": 214.44,
// 			"askSize": 123,
// 			"askTime": 1753707742000,
// 			"bidExchangeCode": "U",
// 			"bidPrice": 214.2,
// 			"bidSize": 157,
// 			"bidTime": 1753707657000,
// 			"eventSymbol": "AAPL:USLF24",
// 			"eventTime": 0,
// 			"sequence": 0,
// 			"timeNanoPart": 0
// 		}
// 	},
// 	"status": "OK"
// }

#[derive(serde::Deserialize)]
struct PriceResponse {
    #[serde(rename = "Quote")]
    quote: serde_json::value::Map<String, serde_json::value::Value>,
}

#[cfg(any(feature = "testnet", feature = "mainnet"))]
pub fn execution_phase() -> Result<()> {
    // Expected to be in the format "symbol,..." (e.g., "fx/XAU/USD" or "equity/AAPL")
    use seda_sdk_rs::HttpFetchOptions;
    let dr_inputs_raw = String::from_utf8(Process::get_inputs())?;

    // If no input is provided, log an error and return.
    if dr_inputs_raw.is_empty() {
        elog!("No input provided for the commodity price request.");
        Process::error("No input provided".as_bytes());
    }

    // If the input is not in the expected format, log an error and return.
    // split at the first /
    let (asset_type, symbol) = dr_inputs_raw.split_once('/').ok_or_else(|| {
        elog!("Invalid input format. Expected format: 'fx/SYMBOL/CURRENCY' or 'equity/SYMBOL'");
        Process::error("Invalid input format".as_bytes());
        anyhow::anyhow!("Invalid input format")
    })?;

    let url = match asset_type {
        "fx" => {
            log!("Fetching commodity price for: {symbol}");
            [API_URL, "cfd/", symbol].concat()
        }
        "equity" => {
            log!("Fetching equity price for: {symbol}");
            [API_URL, "uslf_q/", symbol].concat()
        }
        _ => {
            elog!("Invalid asset type. Expected 'fx' or 'equity'.");
            Process::error("Invalid asset type".as_bytes());
            return Ok(());
        }
    };

    let response = proxy_http_fetch(
        url,
        Some(PROXY_PUBLIC_KEY.to_string()),
        Some(HttpFetchOptions {
            method: seda_sdk_rs::HttpFetchMethod::Get,
            headers: Default::default(),
            body: None,
            timeout_ms: Some(20_000),
        }),
    );

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

    let response_data = serde_json::from_slice::<PriceResponse>(&response.bytes)?;

    // Parse the API response as defined earlier.
    let price = match asset_type {
        "fx" => response_data
            .quote
            .get(&format!("{symbol}:BFX"))
            .and_then(|quote| quote.get("askPrice"))
            .and_then(|price| price.as_f64()),
        "equity" => response_data
            .quote
            .get(&format!("{symbol}:USLF24"))
            .and_then(|quote| quote.get("askPrice"))
            .and_then(|price| price.as_f64()),
        _ => unreachable!(),
    }
    .ok_or_else(|| anyhow::anyhow!("Price not found in response"))?;

    let price_lossless = (price * 100.0) as u128;
    log!("Fetched price: {price_lossless:?}");

    // Report the successful result back to the SEDA network.
    Process::success(&price_lossless.to_le_bytes());

    Ok(())
}
