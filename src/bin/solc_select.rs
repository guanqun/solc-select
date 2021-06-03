use clap::{App, Arg, SubCommand};
use solc_select::{get_available_versions, get_current_version, install_versions, use_version};

fn main() {
    let matches = App::new("solc_select")
        .version("0.1.0")
        .author("Guanqun Lu <guanqunlu@outlook.com>")
        .about("Allows users to install and quickly switch between Solidity compiler versions")
        .subcommand(
            SubCommand::with_name("install")
                .about("list and install available solc versions")
                .arg_from_usage(
                    "[VERSION]... 'specific versions you want to install \"0.8.4\" or \"all\"'",
                ),
        )
        .subcommand(
            SubCommand::with_name("use")
                .about("change the version of global solc compiler")
                .arg_from_usage("<VERSION> 'solc version you want to use (eg: 0.8.4)'"),
        )
        .subcommand(
            SubCommand::with_name("version").about("prints out all installed solc versions"),
        )
        .get_matches();

    match matches.subcommand() {
        ("install", Some(matches)) => {
            let versions: Vec<String> = matches
                .values_of("VERSION")
                .map_or(vec![], |versions| versions.map(|s| s.to_string()).collect());
            install_versions(versions);
        }
        ("use", Some(matches)) => {
            let version = matches.value_of("VERSION").expect("must have VERSION");
            use_version(version);
        }
        ("version", matches) => {
            println!(
                "{}",
                get_current_version().unwrap_or_else(|| "<no-version-installed>".to_string())
            );
        }
        _ => {
            println!("show help");
        }
    }
}
