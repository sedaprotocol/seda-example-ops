use anyhow::Result;
use seda_sdk_rs::{Process, elog, get_reveals, log};

pub fn tally_phase() -> Result<()> {
    // Retrieve consensus reveals from the tally phase.
    let reveals = get_reveals()?;
    let mut revealed_prices: Vec<f64> = Vec::with_capacity(reveals.len());

    for reveal in reveals {
        let price = match serde_json::from_slice::<f64>(&reveal.body.reveal) {
            Ok(price) => price,
            Err(err) => {
                elog!("Failed to parse revealed price: {err}");
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
    let final_price = median(&revealed_prices);
    log!("Final median prices: {final_price:?}");

    // Report the successful result in the tally phase.
    Process::success(&final_price.to_string().as_bytes());

    Ok(())
}

/// Finds the median of a list of prices per price report using f64 values.
fn median(data: &[f64]) -> f64 {
    let mut vals = data.to_vec();
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = vals.len() / 2;

    let median_price = if vals.len() % 2 == 0 {
        (vals[mid - 1] + vals[mid]) / 2.0
    } else {
        vals[mid]
    };

    median_price
}
