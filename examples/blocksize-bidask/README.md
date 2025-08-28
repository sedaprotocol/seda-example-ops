# Blocksize Bid Ask

Deployments:
- [Testnet](https://testnet.explorer.seda.xyz/oracle-programs/67d72c90bff19765b751aaad7b550d7c488390d1e042f02d7395401dece565bb)
<!-- - [Mainnet](https://explorer.seda.xyz/oracle-programs/) -->


## Overview

This Oracle Program gets the price of a price pair using the Blocksize API and returns the price in a format compatible with EVM smart contracts. The API is behind a Data Proxy.

You can test this Oracle Program on testnet with the following command:

```sh
cargo post-dr blocksize-bidask ETHUSD -i 67d72c90bff19765b751aaad7b550d7c488390d1e042f02d7395401dece565bb --gas-price 4000
```

## Execution Phase:

### Input Format

The Execution Phase expects a price pair symbol, and optionally the field from the response to use. By default it will use the `agg_ask_price` field from the API response.

### Process

1. Validates the Data Request execution argument is not empty.
1. Makes a HTTP call to the Blocksize Data Proxy.
1. Converts the decimal to a `u128` with 2 decimal precision.
1. Returns the `u128` in little endian format.

### Examples

#### With No Field

Input: `ETHUSD`

Output: `4537066188`


#### With a Field

Input: `ETHUSD-agg_bid_price`

Output: `4362597230`


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

This supports any price pair supported by the [Blocksize API](https://realtime.blocksize.dev/docs#/Bid%20Ask).