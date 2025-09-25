# Caplight End of Day Market Price

Deployments:
- [Testnet](https://testnet.explorer.seda.xyz/oracle-programs/58e9a7d1c7597e9580f4f44f4e64e3946bff70868f2a6e164da6cfe340a586ee): Non String Result
- [Testnet](https://testnet.explorer.seda.xyz/oracle-programs/a30be1fcb79ca173878f816f3f4bdc73be4511ceb792dd60dbb6db5faba9f50b): String Result
<!-- - [Mainnet]() -->

## Overview

This Oracle Program fetches the latest market data returned from the [Caplight API](https://platform.caplight.com/api/documentation.html#tag/MarketPrice/paths/~1market-price-history/get) and returns the price in a format compatible with EVM smart contracts. It takes a singular ID of a company and then calculates the median among those. The API is behind a Data Proxy.

You can test this Oracle Program on testnet with the following command:

```sh
cargo post-dr caplight-eod-market-price 54782-29 -i 58e9a7d1c7597e9580f4f44f4e64e3946bff70868f2a6e164da6cfe340a586ee -r 3
```

Or for a version that posts the result as a string you can do:

```sh
cargo post-dr caplight-eod-market-price --str-result 54782-29 -i a30be1fcb79ca173878f816f3f4bdc73be4511ceb792dd60dbb6db5faba9f50b
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
