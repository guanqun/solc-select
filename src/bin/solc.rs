use solc_select::{get_artifact_dir, get_current_version};
use std::env::args;
use std::process::Command;

fn main() {
    if let Ok(version) = get_current_version() {
        let mut artifact_dir = get_artifact_dir();
        artifact_dir.push(format!("solc-{}", version));

        Command::new(artifact_dir)
            .args(args().skip(1))
            .status()
            .expect("failed to execute solc file");
    }
}
