<p align="center">
  <a href="https://seda.xyz/">
    <img width="90%" alt="seda-protocol" src="https://raw.githubusercontent.com/sedaprotocol/.github/refs/heads/main/images/banner.png">
  </a>
</p>

<h1 align="center">
  SEDA Oracle Program Templates
</h1>

This oracle template shows official Oracle Program use cases. If this is your first time working with Seda and Oracle Programs I'd recommend reading our [docs](https://docs.seda.xyz/home/) to better understand what an Oracle Program is and how it flows.

In a quick overview:
- **Oracle Program**: Is a WASM(Web Assembly) program that you upload to the Seda network. This program can contain one or both of an Execution and/or Tally phase.
- **Execution Phase**: This is where the logic for what data you want on chain is written. This logic is executed by a number of executors decided by the Data Request poster.
- **Tally Phase**: This is where the logic for consolidating/choosing the best answer is written. For example, mean, median, or whatever else. This logic is executed by Seda nodes before posting the Data Result.
- **Data Request**: A request posted onto a chain by a user that states which Oracle Program to use, the number of executors to run it, and pass the arguments to the program(s). You can choose an Oracle Program for execution and tally, and pass arguments to them separately.
- **Data Proxy**: A proxy to expose a private API to the Seda network without leaking a private key. Calling one comes with a cost configured by the Data Proxy host.

We have the following examples, where you can learn how to quickly post a Data Request specifically for that example by clicking the links, however please check out the [requirements](#requirements) before posting a Data Request with the examples:

- [Caplight Eod Market Price](./examples/caplight-eod-market-price/README.md): A way to ask for the market price history for a company leveraging the Caplight API behind a data proxy.
- [Single Commodity Price](./examples/single-commodity-price/README.md): A way to get the price of a commodity using the DxFeed API behind a Data Proxy.
- [Single Equity Price](./examples/single-commodity-price/README.md): A way to get the price of an equity using the DxFeed API behind a Data Proxy.
- [Multi Price Feed](./examples/multi-price-feed/README.md): A price pair feed using the free APIs for Binance, Mexc, and Okx.
- [Single Price Feed](./examples/single-price-feed/README.md): A price feed using the pro Coingecko API behind a Data Proxy.

If you'd like to learn more about how they work

## Requirements

Make sure the below are installed and in your `PATH` if you want to build, deploy, post Oracle Programs, and post Data Requests. For quick starting, the only requirements are **Bun** and **Rust**.

- **Bun**: Install [Bun](https://bun.sh/) for package management and building.
- **Rust**: Install [Rust](https://rustup.rs/) for development and building.
- **WASM**: Install the [`wasm32-wasip1`](https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1.html) target with `rustup target add wasm32-wasip1` for WASM compilation.
- **WASM-OPT**: Can be installed via `bun/npm/etc install -g binaryren`, `cargo install binaryren`, `cargo binstall binaryren`, or your OS package manager.
- **WABT**: Can be installed via `bun/npm/etc install -g wabt`, or your OS package manager.
- **WASM-STRIP**: Can be install via `cargo install wasm-strip`.

- Alternatively, use the [devcontainer](https://containers.dev/) for a pre-configured environment.

## Getting Started

There are a few example programs in here:

- [Caplight Eod Market Price](./examples/caplight-eod-market-price/README.md): A way to ask for the market price history for a company leveraging the Caplight API behind a data proxy.
- [Single Commodity Price](./examples/single-commodity-price/README.md): A way to get the price of a commodity using the DxFeed API behind a data proxy.
- [Single Equity Price](./examples/single-commodity-price/README.md): A way to get the price of an equity using the DxFeed API behind a data proxy.
- [Multi Price Feed](./examples/multi-price-feed/README.md): A price pair feed using the free APIs for Binance, Mexc, and Okx.
- [Single Price Feed](./examples/single-price-feed/README.md): A price feed using the pro Coingecko API behind a data proxy.

You can interact with the examples in various ways.
To see help information for them all you can run:

```sh
cargo xtask --help
```

You will need to have run `bun install or cargo install-tools` to have the bun dependencies installed. To see the dependencies 

> [!NOTE]
> All commands can also be run via `cargo run <command> <options>`.
> For example `cargo run compile single-price-feed` would work the same as `cargo compile single-price-feed`.

> ![NOTE]
> For some commands, they may require your `seda mnemonic`.
> This is set via an env variable, so you can also use the example [.env](.env.example) file we have here to fill it out.
> There is one for `TESTNET` and `MAINNET`.

### Building

To build one of the Oracle Programs, run the following:

```sh
cargo compile <oracle-program>
```

You can list the program names with

```sh
cargo compile --help
```

where'd you get the output:

```
Usage: xtask compile <ORACLE_PROGRAM>

Arguments:
  <ORACLE_PROGRAM>  [possible values: single-price-feed]

Options:
  -h, --help  Print help
```

### Local Testing

You can test a oracle program with:

```sh
cargo test-op <oracle-program>`
```

where you can optionally pass in a test pattern:

```sh
cargo test-op <oracle-program> <test-pattern>
```

This command will compile the oracle program for you as well before testing it.

You can test all programs at once with:

```sh
cargo test-all-ops
```

### Uploading an Oracle Program

To upload an Oracle Program binary, run:

```sh
cargo deploy <oracle-program> <optional-network>
```

By default the network defaults to `TESTNET`.

This command will compile the oracle program for you as well before uploading it.

> [!IMPORTANT]  
> Make sure you have all the environment variables set in `.env` file.

### Submitting a Data Request

Submitting a Data Request to the SEDA network, run:

```sh
cargo post-dr <oracle-program> [oracle-program-specific-args] -i <oracle-program-id>
```

This will post a transaction and wait till there is an result.

So for example you can do:

```sh
cargo post-dr single-price-feed BTC,ETH -i 2f0c7eea6764398e1e5bf9cde27f206620a89d58b0e37f97cdb6567265c6c2b9 -r 3
```

To post the `single-price-feed` dr example with: the argument `BTC,ETH`, the id, and a replication factor of 3.

> [!IMPORTANT]  
> Make sure you have all the environment variables set in `.env` file.

### Formatting and Linting

The TS side is handled by [Biome](https://biomejs.dev/): `biome format` and `biome format fix`.

The Rust side is handled by `cargo`: `cargo fmt --all -- --check` and `cargo fmt --all`.

Rust additionally has linting via `clippy` with `cargo clippy --all-features --locked -- -D warnings`.

## License

Contents of this repository are open source under the [MIT license](LICENSE).