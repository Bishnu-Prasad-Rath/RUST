mod models;
mod system;
mod monitor;
mod ui;

use models::{ProcessCategory, HealthStatus};
use monitor::enforce_security_and_limits;
use system::{clear_terminal, fetch_live_processes};
use ui::render_dashboard;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let max_allowed_memory = 600; // 600 MB threshold

    loop {
        clear_terminal();
        println!("==========================================================================");
        println!("   🛡️  WINDOWS PROCESS SENTINEL & MINI DEFENDER - LIVE TUI DASHBOARD      ");
        println!("==========================================================================");

        match fetch_live_processes() {
            Ok(mut processes) => {
                // Sort by overall impact score
                processes.sort_by(|a, b| b.impact_score().cmp(&a.impact_score()));

                // Filter top processes to display cleanly in table
                let top_display: Vec<_> = processes.iter().take(15).cloned().collect();

                println!("Active Processes: {} | Max RAM Threshold: {} MB\n", processes.len(), max_allowed_memory);

                // Render styled table
                render_dashboard(&top_display, max_allowed_memory);

                println!("\n--- RUNNING SECURITY SCAN & MEMORY ENFORCEMENT ---");
                enforce_security_and_limits(&mut processes, max_allowed_memory);
            }
            Err(err) => {
                println!("Error fetching process list: {}", err);
            }
        }

        println!("\n[Sentinel sleeping for 5 seconds... Press Ctrl+C to stop]");
        sleep(Duration::from_secs(5));
    }
}