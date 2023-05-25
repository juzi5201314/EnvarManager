fn main() {
    let config = serde_json::from_value::<tauri_utils::config::Config>(tauri_utils::config::parse::read_from(
        std::env::current_dir().unwrap(),
    ).unwrap()).unwrap();
    let version = env!("CARGO_PKG_VERSION");
    assert_eq!(&config.package.version.unwrap_or_else(|| "unknown".to_owned()), version, "Inconsistent versions");
    println!("cargo:rustc-env=__VERSION__={version}");
    println!("cargo:rustc-env=__REPO__={}", env!("CARGO_PKG_REPOSITORY"));
    println!(
        "cargo:rustc-env=GIT_COMMIT_HASH_SHORT={}",
        &git_commit_hash()[..7]
    );

    tauri_build::build()
}

fn git_commit_hash() -> String {
    if let Ok(output) = std::process::Command::new("git")
        .arg("rev-list")
        .arg("-1")
        .arg("HEAD")
        .current_dir("../")
        .output()
    {
        if output.status.success() {
            std::str::from_utf8(&output.stdout[..40])
                .unwrap()
                .to_string()
        } else {
            "UNKNOWN".to_string()
        }
    } else {
        "UNKNOWN".to_string()
    }
}