# Multi Price Feed

Deployments:
- [Testnet]()
<!-- - [Mainnet]() -->

## Execution Phase:

This oracle program takes in one argument for execution:
- A price pair hyphenated symbols i.e. `BTC-USD,ETH-USDT,etc...`

Each executor hits three public APIs: binance, mexc, and okx.

## Tally Phase

It takes no arguments for the tally phase.

The tally phase takes the reveals.
It then takes the median price, before ABI encoding the result, as a `uint256`, and posting it.
This way the result can handily be decoded by an ETH network.