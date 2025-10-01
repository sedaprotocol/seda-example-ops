# Caplight End of Day Market Price

Deployments:
- [Testnet](https://testnet.explorer.seda.xyz/oracle-programs/97ab05dee5f27f8ff1b7dafd4506a5de8924b17535989139c9e8b865ff062e0e)
<!-- - [Mainnet]() -->

## Overview

This Oracle Program fetches the latest market data returned from the [Caplight API](https://platform.caplight.com/api/documentation.html#tag/MarketPrice/paths/~1market-price-history/get) and returns the price in a format compatible with EVM smart contracts. It takes a singular ID of a company and then calculates the median among those. The API is behind a Data Proxy.

You can test this Oracle Program on testnet with the following command:

```sh
cargo post-dr caplight-eod-market-price 54782-29 -i 97ab05dee5f27f8ff1b7dafd4506a5de8924b17535989139c9e8b865ff062e0e
```

## Execution Phase:

### Input Format

The Execution Phase expects the `pitchbookId` of the company.

### Process

1. Validates the Data Request execution argument is not empty.
2. Makes an HTTP call to the Caplight Data Proxy.
3. Converts the decimal to a `u128` with 2 decimal precision.
4. Returns the `u128` in little endian format.

### Example

Input: `54782-29`

Output: `4221`

## Tally Phase

### Input

No additional input is required for this Oracle Program as the Tally Phase only uses the reveals from the Execution Phase.

### Process

1. Collects all price reveals from oracle nodes.
1. Calculates the median price from all the given prices.
1. ABI-encodes the result as a `uint256` for EVM compatibility.
1. Posts the final result.

### Output Format

The result is ABI-encoded as `uint256` where the final number is the median of all the collected price data.

### Example

If execution phase ran with a replication factor of 2 and the prices were:
- 100
- 200

The tally phase would return `150` ABI-encoded as a `uint256`.

## Supported Data

Supports any company available on the Caplight API.
