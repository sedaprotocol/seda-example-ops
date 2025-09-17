use anyhow::Result;
use seda_sdk_rs::{Process, elog, get_reveals, log};

pub fn tally_phase() -> Result<()> {
    // Retrieve consensus reveals from the tally phase.
    let reveals = get_reveals()?;
    let mut revealed_prices: Vec<Vec<u16>> = Vec::with_capacity(reveals.len());

    // Iterate over each reveal, parse its content as an unsigned integer (u16), and store it in the prices array.
    for reveal in reveals {
        let prices = match serde_json::from_slice::<Vec<u16>>(&reveal.body.reveal) {
            Ok(prices) => prices,
            Err(err) => {
                elog!("Failed to parse revealed prices: {err}");
                continue;
            }
        };

        revealed_prices.push(prices);
    }

    if revealed_prices.is_empty() {
        // If no valid prices were revealed, report an error indicating no consensus.
        Process::error("No consensus among revealed results".as_bytes());
        return Ok(());
    }

    // If there are valid prices revealed, calculate the median price from price reports.
    let final_prices = median_each_asset(&revealed_prices);
    log!("Final median prices: {final_prices:?}");

    let final_prices_json = serde_json::to_string(&final_prices)
        .map_err(|err| {
            elog!("Failed to convert final prices to JSON: {err}");
            Process::error("Error converting final prices to JSON".as_bytes());
            anyhow::anyhow!("Failed to convert final prices to JSON: {err}")
        })?;
    // Report the successful result in the tally phase.
    Process::success(&final_prices_json.as_bytes());

    Ok(())
}

/// Finds the median of a list of prices per price report.
fn median_each_asset(data: &[Vec<u16>]) -> Vec<u128> {
    let m = data[0].len();

    if !data.iter().all(|row| row.len() == m) {
        Process::error("Inconsistent row lengths in data reveals".as_bytes());
    }

    let mut result = Vec::with_capacity(m);
    (0..m).for_each(|col| {
        // collect the col-th value from each reveal
        let mut vals = Vec::with_capacity(data.len());
        data.iter().for_each(|row| vals.push(row[col]));
        vals.sort();
        let mid = vals.len() / 2;

        let median_price = if vals.len() % 2 == 0 {
            // safe average of two u16s converted to u128
            let a = vals[mid - 1] as u128;
            let b = vals[mid] as u128;
            (a + b) / 2
        } else {
            vals[mid] as u128
        };

        result.push(median_price);
    });
    result
}