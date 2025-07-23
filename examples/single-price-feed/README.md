# Simple Price Feed

Deployments:
- [Testnet](https://testnet.explorer.seda.xyz/oracle-programs/ea3860e9f537eed6b4ac4ee1c97dea0fd1b7e33a29afaa846ad29671ddaa194e)
<!-- - [Mainnet](https://mainnet.explorer.seda.xyz/oracle-programs/) -->

## Execution Phase:

This oracle program takes in one argument for execution:
- A comma separated list of symbols i.e. `BTC,ETH,etc...`

Each executor hits the coingecko API, configured by a data proxy.
Returning the list of prices in USD in the order the symbols were entered.

## Tally Phase

It takes no arguments for the tally phase.

The tally phase takes the reveals.
It then takes the median of each symbol, before ABI encoding the result, as a `uint256[]`, and posting it. Again keeping the same order as the symbols passed to the execution arguments.
This way the result can handily be decoded by an ETH network.

For example, if you asked for the `BTC,ETH` your result would be `[median_of_BTC, median_of_ETH]` ABI encoded as a `uint256[]`.