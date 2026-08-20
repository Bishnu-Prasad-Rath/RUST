use std::env;
use std::os::windows::process::CommandExt;
use std::process::{self, Command, Stdio};

// Import our existing Win32 engine functions to hunt down the process
use crate::engine::{fetch_live_processes_win32, force_kill_pid};

const DETACHED_PROCESS: u32 = 0x00000008;
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn start_background_daemon() {
    // 1. Locate the exact path of the currently running Sentinel executable
    let exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            println!("❌ Failed to locate executable: {}", e);
            return;
        }
    };

    println!("🚀 Launching Sentinel daemon invisibly in the background...");

    // 2. Spawn a clone of ourselves with the secret --daemon flag
    let mut cmd = Command::new(exe_path);
    cmd.arg("--daemon");
    cmd.creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW);

    // Sever all terminal input/output pipes so it runs completely detached
    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    match cmd.spawn() {
        Ok(child) => {
            println!(
                "✅ Sentinel is now running in the background (PID: {}).",
                child.id()
            );
            println!("💡 You can close this terminal. It will watch for VS Code automatically.");
        }
        Err(e) => println!("❌ Failed to start daemon: {}", e),
    }
}

pub fn stop_background_daemon() {
    println!("🔍 Scanning for background Sentinel processes...");

    let current_pid = process::id();
    let mut killed_any = false;

    // 1. Fetch all running Windows processes
    if let Ok(processes) = fetch_live_processes_win32() {
        for proc in processes {
            let name = proc.name.to_lowercase();

            // 2. Identify Sentinel processes, but EXCLUDE the one currently running the `stop` command
            if (name == "sentinel.exe" || name == "win-process-sentinel.exe")
                && proc.pid != current_pid
            {
                println!(
                    "   └─ 🛑 Terminating Sentinel Daemon (PID: {})...",
                    proc.pid
                );
                if force_kill_pid(proc.pid) {
                    killed_any = true;
                } else {
                    println!("      ❌ Failed to kill PID {}. Access denied.", proc.pid);
                }
            }
        }
    }

    if killed_any {
        println!("✅ All background Sentinel daemons have been stopped.");
    } else {
        println!("⚠️ No background Sentinel processes were found.");
    }
}
