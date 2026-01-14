# Kalshi Simple Feed

Deployments:
- [Testnet](https://testnet.explorer.seda.xyz/oracle-programs/1e6af330f02e7ca41029223da5f79bb7a36d9e6f8043c21baa5f82ded70edbcc)

## Overview

This Oracle Program fetches YES bid prices for a single Kalshi prediction market and provides consensus pricing through median aggregation across oracle nodes.

You can test this Oracle Program on testnet with the following command:

```sh
cargo post-dr kalshi-simple-feed KXPRESPERSON-28-JVAN -i 1e6af330f02e7ca41029223da5f79bb7a36d9e6f8043c21baa5f82ded70edbcc --gas-price 4000
```

## Execution Phase:

### Input Format

The Execution Phase expects a single Kalshi market ticker (e.g., "KXPRESPERSON-28-JVAN").

### Process

1. Validates the Data Request execution argument is not empty.
1. Trims the input to get the market ticker.
1. Makes an HTTP call to the Kalshi API for the specified market.
1. Extracts the `yes_bid` price (in cents) from the market response.
1. Returns the yes_bid price as a string.

### Examples

#### Single Market Query

Input: `KXPRESPERSON-28-JVAN`

Response from Kalshi API:
```json
{
  "market": {
    "yes_bid": 42
  }
}
```

Output: `42` (YES bid price in cents as string)

## Tally Phase

### Input

No additional input is required for this Oracle Program as the Tally Phase only uses the reveals from the Execution Phase.

### Process

1. Collects all price reveals from oracle nodes.
1. Parses each reveal as a u16 integer (YES bid price in cents).
1. Calculates the median price across all oracle node results.
1. Returns the final median price as a string.

### Output Format

The result is the median YES bid price as a string representing the consensus price across oracle nodes.

### Example

If execution phase ran with a replication factor of 3 and the reveals were:
- `42` (Node 1: YES bid = 42¢)
- `40` (Node 2: YES bid = 40¢)
- `44` (Node 3: YES bid = 44¢)

The tally phase would return `42` (median price in cents).

## Supported Data

This Oracle Program supports any prediction market ticker available through the [Kalshi API](https://api.elections.kalshi.com/trade-api/v2/markets/).

For quick reference, the JSON response structure is as follows:

```JSON
{
  "market": {
    "yes_bid": 42
  }
}
```

The program specifically extracts the `yes_bid` field, which represents the current highest bid price for the "YES" outcome in cents (typically 0-100 range for most prediction markets).