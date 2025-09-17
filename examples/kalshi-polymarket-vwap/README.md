# Kalshi-PolyMarket VWAP

Deployments:
- [Testnet](https://testnet.explorer.seda.xyz/oracle-programs/3c59104746354cf7f211ebdb9172aeb57c5c77191bc68376b93acb87a51ae6d6)

## Overview

This Oracle Program fetches YES outcome prices from both Kalshi and PolyMarket prediction markets for the same underlying event, then calculates a Volume-Weighted Average Price (VWAP) between the two platforms to provide a more accurate and balanced market price.

You can test this Oracle Program on testnet with the following command:

```sh
cargo post-dr kalshi-polymarket-vwap KXSB-26-BUF,540209 -i 3c59104746354cf7f211ebdb9172aeb57c5c77191bc68376b93acb87a51ae6d6 --gas-price 4000
```

## Execution Phase:

### Input Format

The Execution Phase expects two comma-separated market identifiers: the first for the Kalshi market ticker, and the second for the corresponding PolyMarket market ID (e.g., "KXSB-26-BUF,540209").

### Process

1. Validates the Data Request execution argument is not empty.
1. Parses the comma-separated input to extract Kalshi and PolyMarket market identifiers.
1. Makes an HTTP call to the Kalshi API to fetch YES bid price and volume.
1. Makes an HTTP call to the PolyMarket API to fetch YES outcome price and volume.
1. Calculates the volume-weighted average price: `VWAP = (Kalshi_Price × Kalshi_Volume + PolyMarket_Price × PolyMarket_Volume) / (Kalshi_Volume + PolyMarket_Volume)`
1. Returns the VWAP as a string.

### Examples

#### Cross-Platform Market Pricing

Input: `KXSB-26-BUF,540209`

Example calculation:
- Kalshi: YES bid = $0.42, Volume = 1000 → Weighted = $420
- PolyMarket: YES price = $0.38, Volume = 1500 → Weighted = $570
- VWAP = ($420 + $570) / (1000 + 1500) = $0.396

Output: `0.39600000` (VWAP as string)

## Tally Phase

### Input

No additional input is required for this Oracle Program as the Tally Phase only uses the reveals from the Execution Phase.

### Process

1. Collects all VWAP reveals from oracle nodes.
1. Parses each reveal as a floating-point VWAP value.
1. Calculates the median VWAP across all oracle node results.
1. Returns the final median VWAP as a string.

### Output Format

The result is the median VWAP value as a string representing the consensus volume-weighted average price across platforms.

### Example

If execution phase ran with a replication factor of 3 and the VWAP results were:
- `0.39600000` (Node 1: VWAP between Kalshi and PolyMarket)
- `0.39800000` (Node 2: VWAP between Kalshi and PolyMarket)
- `0.40000000` (Node 3: VWAP between Kalshi and PolyMarket)

The tally phase would return `0.39800000` (median VWAP).

## Supported Data

This Oracle Program supports prediction markets available on both [Kalshi](https://api.elections.kalshi.com/trade-api/v2/markets/) and [PolyMarket](https://gamma-api.polymarket.com/markets/) platforms, provided they represent the same underlying event.

### API Response Formats

**Kalshi API Response:**
```JSON
{
  "market": {
    "yes_bid_dollars": "0.42",
    "volume": 1000
  }
}
```

**PolyMarket API Response:**
```JSON
{
  "outcomePrices": "[\"0.38\", \"0.62\"]",
  "volume": "1500"
}
```

The program extracts the YES outcome price from both platforms (Kalshi's `yes_bid_dollars` and PolyMarket's first element in `outcomePrices` array) along with their respective volumes to calculate the volume-weighted average price.