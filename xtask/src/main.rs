use std::process::Command;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use xshell::{Cmd, Shell, cmd};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

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

#[derive(Subcommand)]
enum PostableOracleProgram {
    SinglePriceFeed {
        #[clap(help = "Comma-separated list of symbols to fetch prices for (e.g., BTC,ETH)")]
        symbols: String,
    },
}

#[derive(Clone, ValueEnum)]
enum Network {
    Testnet,
    Mainnet,
}

#[derive(Clone, ValueEnum)]
enum PostableNetwork {
    // Ethereum,
    // EthereumTestnet,
    SedaTestnet,
    Seda,
}

#[derive(Subcommand)]
enum Commands {
    Compile {
        oracle_program: OracleProgram,
    },
    Deploy {
        oracle_program: OracleProgram,
        #[arg(value_enum, default_value_t = Network::Testnet)]
        network: Network,
    },
    InstallTools,
    #[clap(alias = "post-dr")]
    PostDataRequest {
        #[clap(global = true, short, long)]
        id: Option<String>,
        #[clap(global = true, short, long)]
        replication_factor: Option<u8>,
        #[arg(short, long, value_enum, default_value_t = PostableNetwork::SedaTestnet)]
        network: PostableNetwork,
        #[command(subcommand)]
        oracle_program: PostableOracleProgram,
    },
    #[clap(alias = "test-op")]
    TestOracleProgram {
        oracle_program: OracleProgram,
        test_name_pattern: Option<String>,
    },
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

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

    let sh = Shell::new()?;

    let cli = Cli::parse();
    match cli.command {
        Commands::Compile { oracle_program } => compile_op(&sh, &oracle_program),
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

fn compile_op(sh: &Shell, oracle_program: &OracleProgram) -> Result<()> {
    let program_name = oracle_program.as_str();

    cmd!(
        sh,
        "cargo build --target wasm32-wasip1 --release -p {program_name}"
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

fn deploy_op(sh: &Shell, network: &Network, oracle_program: &OracleProgram) -> Result<()> {
    match network {
        Network::Testnet => {
            sh.set_var("SEDA_RPC_ENDPOINT", "https://rpc.testnet.seda.xyz");
            sh.set_var("SEDA_EXPLORER_URL", "https://testnet.explorer.seda.xyz");
        }
        Network::Mainnet => {
            sh.set_var("SEDA_RPC_ENDPOINT", "https://rpc.seda.xyz");
            sh.set_var("SEDA_EXPLORER_URL", "https://explorer.seda.xyz");
        }
    }

    compile_op(sh, oracle_program)?;

    let program_name = oracle_program.as_str();
    cmd!(
        sh,
        "bunx seda-sdk oracle-program upload ./target/wasm32-wasip1/release/{program_name}.wasm"
    )
    .run()?;
    Ok(())
}

fn install_tools(sh: &Shell) -> Result<()> {
    // check if bun is installed
    if Command::new("bun").arg("--version").output().is_err() {
        bail!("Bun is not installed. Please install Bun to continue.");
    }

    cmd!(sh, "bun install").run()?;
    Ok(())
}

fn post_dr(
    sh: &Shell,
    id: Option<&str>,
    replication_factor: u8,
    network: &PostableNetwork,
    oracle_program: &PostableOracleProgram,
) -> Result<()> {
    let id = id.ok_or_else(|| anyhow::anyhow!("Oracle program ID is required"))?;

    match network {
        PostableNetwork::SedaTestnet => {
            sh.set_var("SEDA_RPC_ENDPOINT", "https://rpc.testnet.seda.xyz");
            sh.set_var("SEDA_EXPLORER_URL", "https://testnet.explorer.seda.xyz");
        }
        PostableNetwork::Seda => {
            sh.set_var("SEDA_RPC_ENDPOINT", "https://rpc.seda.xyz");
            sh.set_var("SEDA_EXPLORER_URL", "https://explorer.seda.xyz");
        }
    }

    let cmd = sh
        .cmd("bun")
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

fn post_single_price_feed(cmd: Cmd<'_>, symbols: &str) -> std::result::Result<(), anyhow::Error> {
    cmd.arg("--exec-inputs")
        .arg(symbols)
        .arg("--decode-abi")
        .arg("uint256[]")
        .run()?;
    Ok(())
}

fn test_op(
    sh: &Shell,
    oracle_program: &OracleProgram,
    test_name_pattern: Option<&str>,
) -> Result<()> {
    let program_name = oracle_program.as_str();

    compile_op(sh, oracle_program)?;

    let test_path = format!("examples/{program_name}/tests");
    match test_name_pattern {
        Some(pattern) => cmd!(sh, "bun test {test_path} --filter {pattern}").run()?,
        None => cmd!(sh, "bun test {test_path}").run()?,
    }

    Ok(())
}
