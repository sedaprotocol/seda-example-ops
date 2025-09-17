# Kalshi Multi-Market Feed

Deployments:
- [Testnet](https://testnet.explorer.seda.xyz/oracle-programs/92acad211ad071de5b72992503e7f457a4c6e5030fb2db9f8cda0f7882ae5827)

## Overview

This Oracle Program fetches YES bid prices for multiple Kalshi prediction markets simultaneously.

You can test this Oracle Program on testnet with the following command:

```sh
cargo post-dr kalshi-multi-market-feed KXPRESPERSON-28-JVAN,KXPRESPERSON-28-GNEWS -i 92acad211ad071de5b72992503e7f457a4c6e5030fb2db9f8cda0f7882ae5827 --gas-price 4000
```

## Execution Phase:

### Input Format

The Execution Phase expects comma-separated Kalshi market tickers (e.g., "KXGDP-24DEC31,KXGDP-25MAR31").

### Process

1. Validates the Data Request execution argument is not empty.
1. Parses the comma-separated market tickers from input.
1. Makes HTTP calls to the Kalshi API for each market ticker.
1. Extracts the `yes_bid` price (in cents) from each market response.
1. Returns a JSON array of the yes_bid prices as bytes.

### Examples

#### Single Market

Input: `KXGDP-24DEC31`

Output: `[42]` (JSON array with one yes_bid price of 42 cents)

#### Multiple Markets

Input: `KXGDP-24DEC31,KXGDP-25MAR31`

Output: `[42, 38]` (JSON array with yes_bid prices of 42 and 38 cents respectively)

## Tally Phase

### Input

No additional input is required for this Oracle Program as the Tally Phase only uses the reveals from the Execution Phase.

### Process

1. Collects all price array reveals from oracle nodes.
1. Parses each reveal as a JSON array of yes_bid prices (u16 values).
1. Calculates the median price for each market position across all reveals.
1. Converts median prices from u16 to u128 for precision.
1. Returns the final median prices as a JSON string.

### Output Format

The result is a JSON array of median prices (as u128 values) for each market in the same order as the input.

### Example

If execution phase ran with a replication factor of 3 and the reveals were:
- `[42, 38]` (Node 1: KXGDP-24DEC31=42¢, KXGDP-25MAR31=38¢)
- `[40, 36]` (Node 2: KXGDP-24DEC31=40¢, KXGDP-25MAR31=36¢)  
- `[44, 40]` (Node 3: KXGDP-24DEC31=44¢, KXGDP-25MAR31=40¢)

The tally phase would return `[42, 38]` (medians: 42¢ and 38¢ respectively).

## Supported Data

This supports any prediction market ticker available through the [Kalshi API](https://api.elections.kalshi.com/trade-api/v2/markets/).

For quick reference, the JSON response structure is as follows:

```JSON
{
  "market": {
    "yes_bid": 42
  }
}
```

The program specifically extracts the `yes_bid` field, which represents the current highest bid price for the "YES" outcome in cents (0-100 range for most markets).