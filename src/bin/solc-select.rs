use clap::{Parser, Subcommand};
use solc_select::{install_versions, installed_versions, switch_global_version};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// install available solc versions
    Install { versions: Vec<String> },
    /// change the version of global solc compiler
    Use { version: String },
    /// prints out all installed solc versions
    Versions,
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.commands {
        Commands::Install { versions } => {
            install_versions(versions).expect("failed to install version");
        }
        Commands::Use { version } => {
            switch_global_version(&version).expect("failed to switch global version");
        }
        Commands::Versions => {
            if let Ok(installed) = installed_versions() {
                for version in installed {
                    println!("{}", version);
                }
            } else {
                println!("<no-solc-installed>");
            }
        }
    }
}
