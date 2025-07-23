<p align="center">
  <a href="https://seda.xyz/">
    <img width="90%" alt="seda-protocol" src="https://raw.githubusercontent.com/sedaprotocol/.github/refs/heads/main/images/banner.png">
  </a>
</p>

<h1 align="center">
  SEDA Oracle Program Templates
</h1>

This oracle template shows official oracle program use cases. They are run on both `mainnet` and `testnet`.

## Requirements

- **Bun**: Install [Bun](https://bun.sh/) for package management and building.
- **Rust**: Install [Rust](https://rustup.rs/) for development and building.
- **WASM**: Install the [`wasm32-wasip1`](https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1.html) target with `rustup target add wasm32-wasip1` for WASM compilation.

- Alternatively, use the [devcontainer](https://containers.dev/) for a pre-configured environment.

## Getting Started

There are a few example programs in here:

- [Single Price Feed](./examples/single-price-feed/README.md)

You can interact with the examples in various ways.
To see help information for them all you can run:

```sh
cargo xtask --help
```

You will need to have run `bun install or cargo install-tools` to have the bun dependencies installed.

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

## License

Contents of this repository are open source under [EMPTY For Now License](LICENSE).