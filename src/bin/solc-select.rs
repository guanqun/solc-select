use clap::{App, SubCommand};
use solc_select::{install_versions, installed_versions, switch_global_version};

fn main() {
    let matches = App::new("solc_select")
        .version("0.2.0")
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
            SubCommand::with_name("versions").about("prints out all installed solc versions"),
        )
        .get_matches();

    match matches.subcommand() {
        ("install", Some(matches)) => {
            let versions: Vec<String> = matches
                .values_of("VERSION")
                .map_or(vec![], |versions| versions.map(|s| s.to_string()).collect());
            install_versions(versions).expect("failed to install version");
        }
        ("use", Some(matches)) => {
            let version = matches.value_of("VERSION").expect("must have VERSION");
            switch_global_version(version).expect("failed to switch global version");
        }
        ("versions", _) => {
            if let Ok(installed) = installed_versions() {
                for version in installed {
                    println!("{}", version);
                }
            } else {
                println!("<no-solc-installed>");
            }
        }
        _ => {
            println!("show help");
        }
    }
}
