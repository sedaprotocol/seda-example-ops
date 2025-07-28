# Simple Commodity Price

Deployments:
- [Testnet](https://testnet.explorer.seda.xyz/oracle-programs/1434bbb580db612a8de085e1c24d4db2984268ad9bd3c99352dc2b077f674cad)
<!-- - [Mainnet](https://mainnet.explorer.seda.xyz/oracle-programs/ae31c9c4026d259cabab6df4e012f4837175fa27572c49e313337516f971772a) -->

## Execution Phase:

This oracle program takes in one argument for execution:
- A Commodity symbol from the approved list:
  - WTI
  - BRN
  - XAU

Executors get the ask price from the DxFeed API behind a data proxy.

## Tally Phase

It takes no arguments for the tally phase.

The tally phase takes the reveals.
It then takes the median price, before ABI encoding the result, as a `uint256`, and posting it.
This way the result can handily be decoded by an ETH network.