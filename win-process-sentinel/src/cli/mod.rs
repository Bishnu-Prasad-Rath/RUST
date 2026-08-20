pub mod cleaner;
pub mod daemon;
pub mod inventory;
pub mod launcher;
pub mod packages;
pub mod stage;
pub mod workspace;

use crate::engine::process::{fetch_live_processes_win32, force_kill_pid};
use cleaner::run_interactive_cleaner;
use daemon::{start_background_daemon, stop_background_daemon};
use inventory::inspect_installed_apps_and_services;
use launcher::run_apps_and_sites;
use packages::manage_packages_and_drivers;
use stage::{execute_project_stage, init_starter_yaml, trigger_event};
use workspace::{restore_workspace_snapshot, save_workspace_snapshot};

pub fn handle_cli_args(args: Vec<String>) -> bool {
    let is_detached = args.iter().any(|a| a == "--detach" || a == "-d");

    let filtered_args: Vec<String> = args
        .into_iter()
        .filter(|a| a != "--detach" && a != "-d" && a != "--daemon")
        .collect();

    if filtered_args.len() < 2 {
        return false;
    }

    let subcommand = filtered_args[1].to_lowercase();

    match subcommand.as_str() {
        "start" => {
            start_background_daemon();
            true
        }
        "stop" => {
            stop_background_daemon();
            true
        }
        "trigger" => {
            if filtered_args.len() < 3 {
                println!("❌ Usage: sentinel trigger <open|git_push|close> [--detach]");
            } else {
                trigger_event(&filtered_args[2], is_detached);
            }
            true
        }
        "stage" => {
            if filtered_args.len() < 3 {
                println!("❌ Usage: sentinel stage <init|stage_name> [--detach]");
            } else if filtered_args[2] == "init" {
                init_starter_yaml();
            } else {
                execute_project_stage(&filtered_args[2], is_detached);
            }
            true
        }
        "run" => {
            if filtered_args.len() < 3 {
                println!("❌ Usage: sentinel run <app.app,site.web> [--detach]");
            } else {
                let targets = filtered_args[2..].join(" ");
                run_apps_and_sites(&targets, is_detached);
            }
            true
        }
        "snapshot" => {
            if filtered_args.len() < 3 {
                println!("❌ Usage: sentinel snapshot <name>");
            } else {
                save_workspace_snapshot(&filtered_args[2]);
            }
            true
        }
        "restore" => {
            if filtered_args.len() < 3 {
                println!("❌ Usage: sentinel restore <name>");
            } else {
                restore_workspace_snapshot(&filtered_args[2]);
            }
            true
        }
        "inspect" => {
            inspect_installed_apps_and_services();
            true
        }
        "package" => {
            if filtered_args.len() < 3 {
                println!(
                    "❌ Usage: sentinel package <install|update|uninstall|search> <target|--all>"
                );
            } else {
                let target = if filtered_args.len() >= 4 {
                    &filtered_args[3]
                } else {
                    "--all"
                };
                manage_packages_and_drivers(&filtered_args[2], target);
            }
            true
        }
        "clean" | "cleanup" => {
            run_interactive_cleaner();
            true
        }
        "status" => {
            print_quick_status();
            true
        }
        "kill" => {
            if filtered_args.len() < 3 {
                println!("❌ Usage: sentinel kill <process_name_or_pid>");
            } else {
                fuzzy_kill_process(&filtered_args[2]);
            }
            true
        }
        "--help" | "-h" => {
            print_help_menu();
            true
        }
        _ => false,
    }
}

fn print_quick_status() {
    println!("==========================================================================");
    println!("   🛡️  SENTINEL QUICK SYSTEM SNAPSHOT");
    println!("==========================================================================");

    match fetch_live_processes_win32() {
        Ok(processes) => {
            let total_procs = processes.len();
            let total_ram_mb: u64 = processes.iter().map(|p| p.memory_mb).sum();
            let top_app = processes.iter().max_by_key(|p| p.memory_mb);

            println!("📊 Total Running Processes : {}", total_procs);
            println!(
                "💾 Total Managed RAM       : {} MB (~{:.2} GB)",
                total_ram_mb,
                total_ram_mb as f64 / 1024.0
            );
            if let Some(top) = top_app {
                println!(
                    "🔥 Top RAM Consuming App   : {} (PID: {}, {} MB)",
                    top.name, top.pid, top.memory_mb
                );
            }
        }
        Err(e) => println!("Error scanning processes: {}", e),
    }
}

fn fuzzy_kill_process(target: &str) {
    println!("🔍 Searching for process matching '{}'...", target);

    if let Ok(pid) = target.parse::<u32>() {
        if force_kill_pid(pid) {
            println!("✅ Successfully killed PID: {}", pid);
        } else {
            println!("❌ Failed to kill PID {}. Check process permissions.", pid);
        }
        return;
    }

    if let Ok(processes) = fetch_live_processes_win32() {
        let matches: Vec<_> = processes
            .into_iter()
            .filter(|p| p.name.to_lowercase().contains(&target.to_lowercase()))
            .collect();

        if matches.is_empty() {
            println!("⚠️ No running processes found matching '{}'.", target);
            return;
        }

        for proc in matches {
            println!("🎯 Terminating {} (PID: {})...", proc.name, proc.pid);
            if force_kill_pid(proc.pid) {
                println!("   └─ ✅ Terminated");
            } else {
                println!("   └─ ❌ Access Denied");
            }
        }
    }
}

fn print_help_menu() {
    println!("==========================================================================");
    println!("   🛡️  SENTINEL CLI HELP MENU");
    println!("==========================================================================");
    println!("Usage: sentinel [subcommand] [options] [--detach]\n");
    println!("Commands:");
    println!("  trigger <open|git_push|close>       Executes stage mapped to the event");
    println!(
        "  stage <init|stage_name>             Orchestrates project workflow from sentinel.yml"
    );
    println!("  run <app.app,site.web> [--detach]   Launches apps and websites concurrently");
    println!("  snapshot <name>                     Saves open applications to a workspace");
    println!("  restore <name>                      Restores saved workspace");
    println!("  inspect                             Lists installed apps, sizes & services");
    println!("  package <install|update> <target>   Installs/updates packages & drivers");
    println!("  cleanup                             Deep cleans build caches & temp junk");
    println!("  status                              Displays quick system metrics");
    println!("  kill <name|pid>                     Fuzzy kills active processes");
    println!("  --help, -h                          Displays this help menu\n");
}
