use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Resolves the true location of winget.exe on Windows, bypassing PATH issues
fn find_winget_binary() -> Option<PathBuf> {
    // 1. Check direct WindowsApps path in LOCALAPPDATA (This bypasses cmd.exe entirely)
    if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
        let direct_path = Path::new(&local_app_data).join(r"Microsoft\WindowsApps\winget.exe");
        if direct_path.exists() {
            return Some(direct_path);
        }
    }

    // 2. Fallback to assuming it's in the system PATH
    Some(PathBuf::from("winget"))
}

pub fn manage_packages_and_drivers(action: &str, target: &str) {
    let winget_path = match find_winget_binary() {
        Some(path) => path,
        None => {
            println!("❌ Winget is not installed on this system.");
            return;
        }
    };

    match action {
        "update" => {
            if target == "--all" {
                println!("📦 Scanning for all available updates...");
                let mut cmd = Command::new(&winget_path);
                cmd.args(["upgrade", "--all", "--include-unknown"]);
                cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
                let _ = cmd.status();
            } else {
                println!("📦 Updating specific package: {}...", target);
                let mut cmd = Command::new(&winget_path);
                cmd.args(["upgrade", target]);
                cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
                let _ = cmd.status();
            }
        }
        "install" => {
            println!("📦 Installing package: {}...", target);
            let mut cmd = Command::new(&winget_path);
            cmd.args(["install", target]);
            cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
            let _ = cmd.status();
        }
        "uninstall" => {
            println!("🗑️ Uninstalling package: {}...", target);
            let mut cmd = Command::new(&winget_path);
            cmd.args(["uninstall", target]);
            cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
            let _ = cmd.status();
        }
        "search" => {
            println!("🔍 Searching Windows repository for: {}...", target);
            let mut cmd = Command::new(&winget_path);
            cmd.args(["search", target]);
            cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
            let _ = cmd.status();
        }
        _ => {
            println!("❌ Unknown package action: {}", action);
            println!("   Supported actions: update, install, uninstall, search");
        }
    }
}
