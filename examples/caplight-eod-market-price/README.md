# Caplight Eod Market Price

Deployments:
- [Testnet](https://testnet.explorer.seda.xyz/oracle-programs/58e9a7d1c7597e9580f4f44f4e64e3946bff70868f2a6e164da6cfe340a586ee)
<!-- - [Mainnet]() -->

## Overview

This Oracle Program fetches the latest market data returned from the [Caplight API](https://platform.caplight.com/api/documentation.html#tag/MarketPrice/paths/~1market-price-history/get), and returns the price in a format compatible with EVM smart contracts. It takes a singular ID of a company and then calculates the median among those. The API is behind a Data Proxy.

You can test this Oracle Program on testnet with the following command:

```sh
cargo post-dr caplight-eod-market-price 54782-29 -i 58e9a7d1c7597e9580f4f44f4e64e3946bff70868f2a6e164da6cfe340a586ee -r 3
```

## Execution Phase:

### Input Format

The Execution Phase expects the `pitchbookId` of the company.

### Process

1. Gets the input.
1. Makes a HTTP call to the Caplight Data Proxy.
1. Converts the decimal to a `u128` with 2 decimal precision.
1. Returns the `u128` in little endian format.

### Example

Input: `54782-29`
Output: `4069`

## Tally Phase

### Input

No additional input required - uses the reveals from the Execution Phase.

### Process

1. Collects all price reveals from oracle nodes.
1. Calculates the median price from all the given prices.
1. ABI-encodes the result as a `uint256` for EVM compatibility.
1. Posts the final result.

### Output Format

The result is ABI-encoded as `uint256` where the final number is the median of all the collected price data.

### Example

If execution phase ran with a replication factor of 2 and the prices were:
- 4096
- 5000

The tally phase would return `4098` ABI-encoded as a `uint256`.

## Supported Data

Supports any company available on the Caplight API.
