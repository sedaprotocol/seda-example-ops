# EVM Price Feed

Deployments:
- Testnet: TBD

## Overview

This oracle program fetches cryptocurrency prices from Binance API and returns them in a format compatible with EVM smart contracts. It supports multiple trading pairs and calculates median prices across multiple oracle nodes for consensus.

## Execution Phase

### Input Format

The execution phase expects an ABI-encoded array of strings in the format:

```
["BTC-USD","ETH-USD","SOL-USD"]
```

Where each string follows the pattern `{BASE_SYMBOL}-{QUOTE_SYMBOL}`.

### Process

1. Decodes the ABI-encoded input array
2. For each trading pair, fetches the current price from Binance API
3. Converts prices to `u128` with 6 decimal precision
4. Returns the prices as a JSON array

### Example

Input: `["BTC-USD", "ETH-USD"]`
Output: `[45000000000, 2800000000]` (prices in 6 decimal precision)

## Tally Phase

### Input

No additional input required - uses the reveals from the execution phase.

### Process

1. Collects all price reveals from oracle nodes
2. Calculates the median price for each trading pair
3. ABI-encodes the result as `uint256[]` for EVM compatibility
4. Posts the final result

### Output Format

The result is ABI-encoded as `uint256[]` where each element represents the median price of the corresponding trading pair in the input order.

### Example
If the execution phase processed `["BTC-USD", "ETH-USD"]` and the median prices were:
- BTC-USD: $45,000.00 (45000000000 in 6 decimals)
- ETH-USD: $2,800.00 (2800000000 in 6 decimals)

The tally phase would return: `[45000000000, 2800000000]` ABI-encoded as `uint256[]`.

## Supported Trading Pairs

This oracle supports any trading pair available on Binance API.
