use core::fmt;

use inquire::{Select, Text, validator::Validation};
use xshell::{Shell, cmd};

#[derive(clap::Args)]
pub struct NewOracleProgram;

pub enum Language {
    Rust,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::Rust => write!(f, "Rust"),
        }
    }
}

impl Language {
    pub fn select() -> Self {
        Select::new("Select a programming language:", vec![Language::Rust])
            .prompt()
            .unwrap_or(Language::Rust)
    }
}

pub enum Template {
    Empty,
    SingleHTTPFetch,
    MultiHTTPFetch,
    SingleProxyFetch,
    MultiProxyFetch,
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Template::Empty => write!(f, "Empty"),
            Template::SingleHTTPFetch => write!(f, "Single HTTP Fetch"),
            Template::MultiHTTPFetch => write!(f, "Multi HTTP Fetch"),
            Template::SingleProxyFetch => write!(f, "Single Proxy Fetch"),
            Template::MultiProxyFetch => write!(f, "Multi Proxy Fetch"),
        }
    }
}

impl Template {
    pub fn select() -> Self {
        Select::new(
            "Select a template for the new oracle program:",
            vec![
                Template::Empty,
                Template::SingleHTTPFetch,
                Template::MultiHTTPFetch,
                Template::SingleProxyFetch,
                Template::MultiProxyFetch,
            ],
        )
        .prompt()
        .unwrap_or(Template::Empty)
    }

    pub fn add_template_deps(&self, shell: &Shell) -> anyhow::Result<()> {
        match self {
            Template::Empty => Ok(()),
            Template::SingleHTTPFetch | Template::MultiHTTPFetch => {
                cmd!(shell, "cargo add serde-json").run()?;
                Ok(())
            }
            Template::SingleProxyFetch | Template::MultiProxyFetch => {
                cmd!(shell, "cargo add serde-json").run()?;
                Ok(())
            }
        }
    }
}

pub enum DataHandler {
    Empty,
    Average,
    Median,
}

impl fmt::Display for DataHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataHandler::Empty => write!(f, "Empty"),
            DataHandler::Average => write!(f, "Average"),
            DataHandler::Median => write!(f, "Median"),
        }
    }
}

impl DataHandler {
    pub fn select() -> Self {
        Select::new(
            "Select a data handler for the new oracle program:",
            vec![
                DataHandler::Empty,
                DataHandler::Average,
                DataHandler::Median,
            ],
        )
        .prompt()
        .unwrap_or(DataHandler::Empty)
    }
}

pub enum Encoding {
    EthAbi,
    Json,
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Encoding::EthAbi => write!(f, "Ethereum ABI"),
            Encoding::Json => write!(f, "JSON"),
        }
    }
}

impl Encoding {
    pub fn select() -> Self {
        Select::new(
            "Select an encoding for the new oracle program:",
            vec![Encoding::EthAbi, Encoding::Json],
        )
        .prompt()
        .unwrap_or(Encoding::EthAbi)
    }

    pub fn add_encoding_deps(&self, shell: &Shell) -> anyhow::Result<()> {
        match self {
            Encoding::EthAbi => {
                cmd!(shell, "cargo add ethabi").run()?;
                Ok(())
            }
            Encoding::Json => {
                cmd!(shell, "cargo add serde serde-json").run()?;
                Ok(())
            }
        }
    }
}

pub fn is_valid_pkg_name(project_name: &str) -> bool {
    let mut chars = project_name.chars().peekable();
    !project_name.is_empty()
        && !chars.peek().map(|c| c.is_ascii_digit()).unwrap_or_default()
        && !chars.any(|ch| !(ch.is_alphanumeric() || ch == '-' || ch == '_') || ch.is_uppercase())
}

const MAIN_TEMPLATE: &str = r#"
use execution_phase::execution_phase;
use seda_sdk_rs::oracle_program;
use tally_phase::tally_phase;

mod execution_phase;
mod tally_phase;

#[oracle_program]
impl NewOracleProgram {
    fn execute() {
        execution_phase().unwrap();
    }

    fn tally() {
        tally_phase().unwrap();
    }
}"#;

impl NewOracleProgram {
    pub fn handle(self, shell: &Shell) -> anyhow::Result<()> {
        // Some basic validation for the project name to disallow empty names.
        let project_name_validator = |input: &str| {
            if input.is_empty() {
                Ok(Validation::Invalid("Project name cannot be empty".into()))
            } else {
                Ok(Validation::Valid)
            }
        };

        let project_name = Text::new("Enter the name of the new oracle program:")
            .with_placeholder("average-crypto-price-feed")
            .with_validator(project_name_validator)
            .prompt()?;

        let project_name = if is_valid_pkg_name(&project_name) {
            project_name
        } else {
            project_name
                .trim()
                .to_lowercase()
                .replace([':', ';', ' ', '~'], "-")
                .replace(['.', '\\', '/'], "")
        };

        let language = Language::select();
        let template = Template::select();
        let _data_handler = DataHandler::select();
        let encoding = Encoding::select();

        match language {
            Language::Rust => {
                println!("Creating a new Rust oracle program: {project_name}");
                cmd!(shell, "cargo new --bin examples/{project_name}").run()?;
                shell.change_dir(format!("examples/{project_name}"));
                cmd!(shell, "touch src/execution_phase.rs src/tally_phase.rs").run()?;
                cmd!(shell, "cargo add anyhow seda-sdk-rs").run()?;
                template.add_template_deps(shell)?;
                encoding.add_encoding_deps(shell)?;
                shell.write_file("src/main.rs", MAIN_TEMPLATE)?;
                cmd!(shell, "cargo fmt --all").run()?;
                println!("New oracle program created successfully.");
            }
        }
        Ok(())
    }
}
