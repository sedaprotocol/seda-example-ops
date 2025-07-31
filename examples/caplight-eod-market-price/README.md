# Caplight Eod Market Price

Deployments:
- [Testnet](https://testnet.explorer.seda.xyz/oracle-programs/58e9a7d1c7597e9580f4f44f4e64e3946bff70868f2a6e164da6cfe340a586ee)
<!-- - [Mainnet]() -->

## Execution Phase:

This oracle program takes in one argument for execution a `pitchbookId` as expected by the [Caplight API](https://platform.caplight.com/api/documentation.html#tag/MarketPrice/paths/~1market-price-history/get).
Executors get the market data price from the Caplight API behind a data proxy.

## Tally Phase

It takes no arguments for the tally phase.

The tally phase takes the reveals.
It then takes the median price, before ABI encoding the result, as a `uint256`, and posting it.
This way the result can handily be decoded by an ETH network.