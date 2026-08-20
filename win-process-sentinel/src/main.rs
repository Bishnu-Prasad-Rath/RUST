mod cli;
mod engine;
mod models;
mod monitor;
mod ui;

use cli::handle_cli_args;
use engine::{
    clear_terminal, fetch_live_processes_win32, get_system_memory_metrics,
    start_global_hotkey_listener, FreezeTracker,
};
use monitor::enforce_security_and_limits;
use ui::render_dashboard;

use std::collections::HashSet; // 🆕 Added to prevent spam-triggering
use std::env;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};
use sysinfo::System; // 🆕 Added to read VS Code paths dynamically

// --- NEW IMPORTS FOR WINDOWS ANSI SUPPORT ---
use windows_sys::Win32::System::Console::{
    GetConsoleMode, GetStdHandle, SetConsoleMode, ENABLE_VIRTUAL_TERMINAL_PROCESSING,
    STD_OUTPUT_HANDLE,
};

/// Enables ANSI escape sequence processing on older Windows consoles
fn enable_windows_ansi_support() {
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle != 0 {
            let mut mode: u32 = 0;
            if GetConsoleMode(handle, &mut mode) != 0 {
                let _ = SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
            }
        }
    }
}

fn main() {
    enable_windows_ansi_support();

    let args: Vec<String> = env::args().collect();

    // 1. Detect if we are running in invisible background mode
    let is_daemon = args.iter().any(|a| a == "--daemon");

    if handle_cli_args(args.clone()) {
        return;
    }

    let freeze_threshold_secs = 15;
    let mut freeze_tracker = FreezeTracker::new();
    start_global_hotkey_listener();

    let mut last_ui_update = Instant::now();
    let mut last_cleanup = Instant::now();
    let mut force_draw = true;

    // 🆕 Initialize the system reader and workspace tracker outside the loop
    let mut sys = System::new_all();
    let mut triggered_workspaces: HashSet<PathBuf> = HashSet::new();

    if !is_daemon {
        println!("🚀 Sentinel TUI started. Waiting for VS Code...");
    }

    loop {
        // ⚡ LOGIC TICK 1: Dynamic VS Code Path Resolution
        sys.refresh_processes();
        for (_pid, process) in sys.processes() {
            // Safely check the process name (handles sysinfo v0.30 OsStr changes safely)
            if process.name().contains("Code.exe") {
                // Loop through VS Code's launch arguments to find the folder path
                for arg in process.cmd() {
                    let target_dir = PathBuf::from(arg);

                    if target_dir.is_dir() {
                        let dynamic_yaml_path = target_dir.join("sentinel.yml");

                        // If the YAML exists AND we haven't triggered it yet this session
                        if dynamic_yaml_path.exists()
                            && !triggered_workspaces.contains(&dynamic_yaml_path)
                        {
                            // Mark as triggered so we don't open multiple duplicate tabs
                            triggered_workspaces.insert(dynamic_yaml_path.clone());

                            // 1. Switch working directory to the dynamically opened project folder
                            let _ = std::env::set_current_dir(&target_dir);

                            // 2. Trigger the "open" event configured in that project's sentinel.yml
                            cli::stage::trigger_event("open", false);
                        }
                    }
                }
            }
        }

        // ⚡ LOGIC TICK 2: Your existing active workspace tracker
        freeze_tracker.poll_active_ide_workspace();

        // 🧹 CLEANUP TICK: Every 5 seconds, clear closed workspaces from memory
        if last_cleanup.elapsed() >= Duration::from_secs(5) {
            freeze_tracker.cleanup_closed_workspaces();
            last_cleanup = Instant::now();
        }

        // 🐢 RENDER TICK: Only draw the TUI if we are NOT in daemon mode
        if !is_daemon && (force_draw || last_ui_update.elapsed() >= Duration::from_secs(5)) {
            clear_terminal();

            let mem_metrics = get_system_memory_metrics();
            println!("==========================================================================");
            println!("   🛡️  WINDOWS PROCESS SENTINEL & MINI DEFENDER - LIVE TUI DASHBOARD      ");
            println!("==========================================================================");
            println!(
                "💻 System RAM: {} MB Total | {} MB Free ({}% Load)",
                mem_metrics.total_ram_mb, mem_metrics.available_ram_mb, mem_metrics.memory_load_pct
            );

            match fetch_live_processes_win32() {
                Ok(mut processes) => {
                    enforce_security_and_limits(
                        &mut processes,
                        mem_metrics.dynamic_app_threshold_mb,
                    );

                    processes.sort_by(|a, b| b.memory_mb.cmp(&a.memory_mb));
                    let top_display: Vec<_> = processes.iter().take(15).cloned().collect();

                    render_dashboard(&top_display, mem_metrics.dynamic_app_threshold_mb);
                    let killed_pids = freeze_tracker.scan_and_enforce(freeze_threshold_secs);
                    if !killed_pids.is_empty() {
                        println!("Auto-terminated frozen PIDs: {:?}", killed_pids);
                    }
                }
                Err(err) => println!("Error fetching process list: {}", err),
            }

            println!("\n[Sentinel sleeping... UI updates every 5s, Triggers instantly]");
            println!("[Press Ctrl + Alt + Shift + Esc anytime to Panic-Kill active window]");

            last_ui_update = Instant::now();
            force_draw = false;
        }

        sleep(Duration::from_millis(500));
    }
}
