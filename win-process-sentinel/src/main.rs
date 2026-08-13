mod cli;
mod freeze_watchdog;
mod models;
mod monitor;
mod system;
mod ui; // <--- Declare new module

use cli::handle_cli_args;
use freeze_watchdog::{FreezeTracker, start_global_hotkey_listener};
use std::env;
use std::thread::sleep;
use std::time::Duration;
use system::{clear_terminal, fetch_live_processes_win32};
use ui::render_dashboard;

fn main() {
    // 1. Capture CLI arguments
    let args: Vec<String> = env::args().collect();

    // 2. If a subcommand was passed (e.g. `cargo run -- status`), execute it and exit
    if handle_cli_args(args) {
        return;
    }

    // 3. If no subcommand was passed, launch full TUI dashboard
    let max_allowed_memory = 600;
    let freeze_threshold_secs = 15;
    let mut freeze_tracker = FreezeTracker::new();

    start_global_hotkey_listener();

    loop {
        clear_terminal();
        println!("==========================================================================");
        println!("   🛡️  WINDOWS PROCESS SENTINEL & MINI DEFENDER - LIVE TUI DASHBOARD      ");
        println!("==========================================================================");

        match fetch_live_processes_win32() {
            Ok(mut processes) => {
                processes.sort_by(|a, b| b.memory_mb.cmp(&a.memory_mb));
                let top_display: Vec<_> = processes.iter().take(15).cloned().collect();

                println!(
                    "Active Processes: {} | Max RAM Threshold: {} MB\n",
                    processes.len(),
                    max_allowed_memory
                );

                render_dashboard(&top_display, max_allowed_memory);

                println!("\n--- RUNNING SECURITY SCAN & WATCHDOG ENFORCEMENT ---");

                let killed_pids = freeze_tracker.scan_and_enforce(freeze_threshold_secs);
                if !killed_pids.is_empty() {
                    println!("Auto-terminated frozen PIDs: {:?}", killed_pids);
                }
            }
            Err(err) => {
                println!("Error fetching process list: {}", err);
            }
        }

        println!("\n[Sentinel sleeping for 5 seconds... Press Ctrl+C to stop]");
        println!("[Press Ctrl + Alt + Shift + Esc anytime to Panic-Kill active window]");

        sleep(Duration::from_secs(5));
    }
}
