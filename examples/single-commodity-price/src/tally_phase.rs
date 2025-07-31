use anyhow::Result;
use ethabi::{Token, ethereum_types::U256};
use seda_sdk_rs::{Process, elog, get_reveals, log};

/**
 * Executes the tally phase within the SEDA network.
 * This phase aggregates the results (e.g., price data) revealed during the execution phase,
 * calculates the median value, and submits it as the final result.
 * Note: The number of reveals depends on the replication factor set in the data request parameters.
 */
pub fn tally_phase() -> Result<()> {
    // Tally inputs can be retrieved from Process.getInputs(), though it is unused in this example.
    // let tally_inputs = Process::get_inputs();

    // Retrieve consensus reveals from the tally phase.
    let reveals = get_reveals()?;
    let mut revealed_prices: Vec<u128> = Vec::with_capacity(reveals.len());

    // Iterate over each reveal, parse its content as an unsigned integer (u128), and store it in the prices array.
    for reveal in reveals {
        let price = match serde_json::from_slice::<[u8; 16]>(&reveal.body.reveal) {
            Ok(price) => u128::from_le_bytes(price),
            Err(err) => {
                elog!("Failed to parse revealed prices: {err}");
                continue;
            }
        };

        revealed_prices.push(price);
    }

    if revealed_prices.is_empty() {
        // If no valid prices were revealed, report an error indicating no consensus.
        Process::error("No consensus among revealed results".as_bytes());
        return Ok(());
    }

    // If there are valid prices revealed, calculate the median price from price reports.
    let final_prices = median(&revealed_prices);
    log!("Final median prices: {final_prices:?}");

    let result = ethabi::encode(&[final_prices]);

    // Report the successful result in the tally phase, encoding the result as bytes.
    // Encoding result with big endian to decode from EVM contracts.
    Process::success(&result);

    Ok(())
}

/// Finds the median of a list of prices per price report.
fn median(data: &[u128]) -> Token {
    let m = data.len();

    if m == 0 {
        Process::error("No valid data available for median calculation".as_bytes());
    }

    let mut sorted_data = data.to_vec();
    sorted_data.sort_unstable();

    let median_price = if m % 2 == 0 {
        // safe average of two u128s without overflow
        let a = sorted_data[m / 2 - 1];
        let b = sorted_data[m / 2];
        a.midpoint(b)
    } else {
        sorted_data[m / 2]
    };

    // convert to Token::Uint for encoding
    Token::Uint(U256::from(median_price))
}
