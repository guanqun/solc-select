use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

fn get_artifact_dir() -> PathBuf {
    let mut artifacts_dir = dirs::home_dir().expect("failed to get home folder");
    artifacts_dir.push(".solc-select");
    artifacts_dir.push("artifacts");
    artifacts_dir
}

fn get_global_version_path() -> PathBuf {
    let mut global_version_path = dirs::home_dir().expect("failed to get home folder");
    global_version_path.push("global-version");
    global_version_path
}

pub fn install_versions(to_install_versions: Vec<String>) {
    let all = get_available_versions();

    if to_install_versions.is_empty() {
        println!("Available versions to install:");

        for version in all.keys() {
            println!("{}", version);
        }
    } else {
        let artifact_dir = get_artifact_dir();
        std::fs::create_dir_all(artifacts_dir).expect("failed to create artifacts.");

        for (version, artifact) in &all {
            if to_install_versions.contains(&"all".to_string())
                || to_install_versions.contains(version)
            {
                println!("Installing '{}'...", version);

                let url = format!(
                    "https://binaries.soliditylang.org/{}/{}",
                    soliditylang_platform(),
                    artifact
                );
                let bytes = reqwest::blocking::get(url)
                    .expect("failed to get artifact file")
                    .bytes()
                    .expect("failed to get artifact file");

                let mut artifact_file = get_artifact_dir();
                artifact_file.push(format!("solc-{}", version));

                std::fs::write(&artifact_file, bytes)
                    .expect("failed to write artifact file to disk.");

                // make it executable
                let mut perms = std::fs::metadata(&artifact_file)
                    .expect("failed to get file metadata")
                    .permissions();
                perms.set_mode(0o775);
                std::fs::set_permissions(&artifact_file, perms)
                    .expect("failed to set to executable");

                println!("Version '{}' installed.", version);
            }
        }
    }
}

pub fn use_version(version: &str) -> Result<()> {
    if installed_versions()?.contains(&version.to_string()) {
        let global_version_path = get_global_version_path();
        std::fs::write(global_version_path, version).expect("failed to write global-version");
        println!("Switched global version to {}", version);
    } else if get_available_versions()
        .keys()
        .any(|v| v.as_str() == version)
    {
        println!(
            "You need to install '{}' prior to using it. Use `solc-select install {}`",
            version, version
        );
    } else {
        println!("Unknown version `{}`", version);
    }

    Ok(())
}

pub fn get_current_version() -> Option<String> {
    let mut global_version_path = dirs::home_dir().expect("failed to get home folder");
    global_version_path.push("global-version");

    std::fs::read_to_string(global_version_path)
        .map(|s| s.trim().to_string())
        .ok()
}

fn installed_versions() -> Result<Vec<String>> {
    let artifacts_dir = get_artifact_dir();

    let mut versions = vec![];
    for entry in std::fs::read_dir(artifacts_dir)? {
        let entry = entry?;
        if let Some(version) = entry.path().to_string_lossy().strip_prefix("solc-") {
            versions.push(version.to_string());
        }
    }
    Ok(versions)
}

pub fn get_available_versions() -> HashMap<String, String> {
    let url = format!(
        "https://binaries.soliditylang.org/{}/list.json",
        soliditylang_platform()
    );

    #[derive(Deserialize)]
    struct Response {
        releases: HashMap<String, String>,
    }

    let response: Response = reqwest::blocking::get(&url)
        .expect("failed to get releases")
        .json()
        .expect("failed to get releases");

    response.releases
}

fn soliditylang_platform() -> &'static str {
    match sys_info::os_type().unwrap_or("".into()).as_str() {
        "Linux" => "linux-amd64",
        "Darwin" => "macosx-amd64",
        _ => panic!("Unsupported platform."),
    }
}
