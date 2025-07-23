use std::process::Command;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use xshell::{Cmd, Shell, cmd};

/// A command-line tool for managing  the example SEDA oracle programs.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// The oracle programs that can be managed.
#[derive(Clone, ValueEnum)]
enum OracleProgram {
    SinglePriceFeed,
}

impl OracleProgram {
    fn as_str(&self) -> &str {
        match self {
            OracleProgram::SinglePriceFeed => "single-price-feed",
        }
    }
}

/// The oracle programs that can have a data request posted to a network.
#[derive(Subcommand)]
enum PostableOracleProgram {
    SinglePriceFeed {
        #[clap(help = "Comma-separated list of symbols to fetch prices for (e.g., BTC,ETH)")]
        symbols: String,
    },
}

/// The networks that the oracle programs can be compiled and deployed to.
#[derive(Clone, ValueEnum)]
enum SedaNetwork {
    Testnet,
    Mainnet,
}

impl SedaNetwork {
    fn as_str(&self) -> &str {
        match self {
            SedaNetwork::Testnet => "testnet",
            SedaNetwork::Mainnet => "mainnet",
        }
    }
}

/// The networks that can have a data request posted to them.
/// Note: Currently, only Seda networks are supported for posting data requests.
#[derive(Clone, ValueEnum)]
enum PostableNetwork {
    // Ethereum,
    // EthereumTestnet,
    SedaTestnet,
    Seda,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile an oracle program for a specific Seda network.
    Compile {
        /// The oracle program to compile.
        oracle_program: OracleProgram,
        /// The Seda network to compile the oracle program for.
        #[arg(value_enum, default_value_t = SedaNetwork::Testnet)]
        network: SedaNetwork,
    },
    /// Deploy an oracle program to a specific Seda network.
    Deploy {
        /// The oracle program to deploy.
        oracle_program: OracleProgram,
        /// The Seda network to deploy the oracle program to.
        #[arg(value_enum, default_value_t = SedaNetwork::Testnet)]
        network: SedaNetwork,
    },
    /// Install necessary tools for working with SEDA oracle programs.
    InstallTools,
    /// Post a data request for a specified oracle program on a network.
    #[clap(alias = "post-dr")]
    PostDataRequest {
        /// The ID of the oracle program to post the data request for.
        /// This is required and should be the program ID as a string.
        #[clap(global = true, short, long)]
        id: Option<String>,
        /// The replication factor for the data request.
        /// This is optional and defaults to 1 if not provided.
        #[clap(global = true, short, long)]
        replication_factor: Option<u8>,
        /// The network to post the data request to.
        #[arg(short, long, value_enum, default_value_t = PostableNetwork::SedaTestnet)]
        network: PostableNetwork,
        /// The oracle program to post the data request for.
        #[command(subcommand)]
        oracle_program: PostableOracleProgram,
    },
    /// Test an oracle program.
    #[clap(alias = "test-op")]
    TestOracleProgram {
        /// The oracle program to test.
        oracle_program: OracleProgram,
        /// The test name pattern to use.
        test_name_pattern: Option<String>,
    },
}

/// The main entry point where we exit with an error if any operation fails.
fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

/// The main function that executes the command-line interface for managing SEDA oracle programs.
fn try_main() -> Result<()> {
    // Ensure our working directory is the toplevel
    {
        let toplevel_path = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()
            .with_context(|| "Invoking git rev-parse")?;
        if !toplevel_path.status.success() {
            bail!("Failed to invoke git rev-parse");
        }
        let path = String::from_utf8(toplevel_path.stdout)?;
        std::env::set_current_dir(path.trim()).with_context(|| "Changing to toplevel")?;
    }

    dotenvy::dotenv()?;

    let sh = Shell::new()?;
    let cli = Cli::parse();
    match cli.command {
        Commands::Compile {
            oracle_program,
            network,
        } => compile_op(&sh, &oracle_program, &network),
        Commands::Deploy {
            oracle_program,
            network,
        } => deploy_op(&sh, &network, &oracle_program),
        Commands::InstallTools => install_tools(&sh),
        Commands::PostDataRequest {
            id,
            replication_factor,
            oracle_program,
            network,
        } => post_dr(
            &sh,
            id.as_deref(),
            replication_factor.unwrap_or(1),
            &network,
            &oracle_program,
        ),
        Commands::TestOracleProgram {
            oracle_program,
            test_name_pattern,
        } => test_op(&sh, &oracle_program, test_name_pattern.as_deref()),
    }
}

/// Compile a specified oracle program for a specific Seda network.
fn compile_op(
    sh: &Shell,
    oracle_program: &OracleProgram,
    seda_network: &SedaNetwork,
) -> Result<()> {
    let program_name = oracle_program.as_str();
    let seda_network = seda_network.as_str();

    cmd!(
        sh,
        "cargo build --target wasm32-wasip1 --release -p {program_name} --no-default-features --features {seda_network}"
    )
    .run()?;
    cmd!(
        sh,
        "wasm-strip target/wasm32-wasip1/release/{program_name}.wasm"
    )
    .run()?;
    cmd!(sh, "wasm-opt -Oz --enable-bulk-memory target/wasm32-wasip1/release/{program_name}.wasm -o target/wasm32-wasip1/release/{program_name}.wasm").run()?;
    Ok(())
}

/// Deploy a specified oracle program to a Seda network.
fn deploy_op(sh: &Shell, seda_network: &SedaNetwork, oracle_program: &OracleProgram) -> Result<()> {
    // These env vars are used by the `seda-sdk` CLI tool to connect to the Seda network.
    let (rpc, explorer, mnemonic) = match seda_network {
        SedaNetwork::Testnet => (
            "https://rpc.testnet.seda.xyz",
            "https://testnet.explorer.seda.xyz",
            std::env::var("SEDA_MNEMONIC_TESTNET")?,
        ),
        SedaNetwork::Mainnet => (
            "https://rpc.seda.xyz",
            "https://explorer.seda.xyz",
            std::env::var("SEDA_MNEMONIC_MAINNET")?,
        ),
    };

    compile_op(sh, oracle_program, seda_network)?;

    let program_name = oracle_program.as_str();
    cmd!(
        sh,
        "bunx seda-sdk oracle-program upload ./target/wasm32-wasip1/release/{program_name}.wasm"
    )
    .env("SEDA_RPC_ENDPOINT", rpc)
    .env("SEDA_EXPLORER_URL", explorer)
    .env("SEDA_MNEMONIC", mnemonic)
    .run()?;
    Ok(())
}

/// Install necessary tools for working with SEDA oracle programs.
fn install_tools(sh: &Shell) -> Result<()> {
    // check if bun is installed
    if Command::new("bun").arg("--version").output().is_err() {
        bail!("Bun is not installed. Please install Bun to continue.");
    }

    cmd!(sh, "bun install").run()?;
    Ok(())
}

/// Post a data request for a specified oracle program on a network.
/// With the specified ID and replication factor.
fn post_dr(
    sh: &Shell,
    id: Option<&str>,
    replication_factor: u8,
    network: &PostableNetwork,
    oracle_program: &PostableOracleProgram,
) -> Result<()> {
    let id = id.ok_or_else(|| anyhow::anyhow!("Oracle program ID is required"))?;

    let (rpc, explorer, mnemonic) = match network {
        PostableNetwork::SedaTestnet => (
            "https://rpc.testnet.seda.xyz",
            "https://testnet.explorer.seda.xyz",
            std::env::var("SEDA_MNEMONIC_TESTNET")?,
        ),
        PostableNetwork::Seda => (
            "https://rpc.seda.xyz",
            "https://explorer.seda.xyz",
            std::env::var("SEDA_MNEMONIC_MAINNET")?,
        ),
    };

    let cmd = sh
        .cmd("bun")
        .env("SEDA_RPC_ENDPOINT", rpc)
        .env("SEDA_EXPLORER_URL", explorer)
        .env("SEDA_MNEMONIC", mnemonic)
        .arg("run")
        .arg("./scripts/post-dr.ts")
        .arg("--oracle-program-id")
        .arg(id)
        .arg("--replication-factor")
        .arg(replication_factor.to_string());

    match oracle_program {
        PostableOracleProgram::SinglePriceFeed { symbols } => post_single_price_feed(cmd, symbols),
    }
}

/// Post a single price feed data request with the specified symbols.
fn post_single_price_feed(cmd: Cmd<'_>, symbols: &str) -> std::result::Result<(), anyhow::Error> {
    cmd.arg("--exec-inputs")
        .arg(symbols)
        .arg("--decode-abi")
        .arg("uint256[]")
        .run()?;
    Ok(())
}

/// Test an oracle program, optionally filtering tests by a name pattern.
fn test_op(
    sh: &Shell,
    oracle_program: &OracleProgram,
    test_name_pattern: Option<&str>,
) -> Result<()> {
    let program_name = oracle_program.as_str();

    // We always test against the testnet feature flag- it doesn't matter which network we compiled for.
    // Since the tests are run against the compiled program and mocking when necessary.
    compile_op(sh, oracle_program, &SedaNetwork::Testnet)?;

    let test_path = format!("examples/{program_name}/tests");
    match test_name_pattern {
        Some(pattern) => {
            dbg!(&pattern);
            cmd!(sh, "bun test {test_path} -t {pattern}").run()?
        }
        None => cmd!(sh, "bun test {test_path}").run()?,
    }

    Ok(())
}
