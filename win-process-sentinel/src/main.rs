mod models;
mod system;
mod monitor;
mod ui;
mod freeze_watchdog;

use freeze_watchdog::{FreezeTracker, start_global_hotkey_listener};
use system::{clear_terminal, fetch_live_processes_win32};
use ui::render_dashboard;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let max_allowed_memory = 600; // 600 MB threshold
    let freeze_threshold_secs = 15; // 15-second Unresponsive Limit
    let mut freeze_tracker = FreezeTracker::new();

    // ⚡ START GLOBAL HOTKEY LISTENER IN BACKGROUND THREAD
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

                println!("Active Processes: {} | Max RAM Threshold: {} MB\n", processes.len(), max_allowed_memory);

                render_dashboard(&top_display, max_allowed_memory);

                println!("\n--- RUNNING SECURITY SCAN & WATCHDOG ENFORCEMENT ---");
                
                // Scan for frozen windows (>15 seconds unresponsive)
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