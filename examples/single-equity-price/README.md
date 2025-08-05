# Simple Equity Price

Deployments:
- [Testnet](https://testnet.explorer.seda.xyz/oracle-programs/02b2e01a9dd08eee1b1c7976ebcb382a7dc75368e073266fb70a1ed259435848)
<!-- - [Mainnet](https://mainnet.explorer.seda.xyz/oracle-programs/ae31c9c4026d259cabab6df4e012f4837175fa27572c49e313337516f971772a) -->

## Execution Phase:

This oracle program takes in one argument for execution:
- A equity symbol from the approved list:
  - SPY
  - TSLA
  - MSFT
  - AAPL
  - AMZN
  - NVDA
  - GOOG
  - META
  - UNH
  - VAPE

Executors get the ask price from the DxFeed API behind a data proxy.

## Tally Phase

It takes no arguments for the tally phase.

The tally phase takes the reveals.
It then takes the median price, before ABI encoding the result, as a `uint256`, and posting it.
This way the result can handily be decoded by an ETH network.