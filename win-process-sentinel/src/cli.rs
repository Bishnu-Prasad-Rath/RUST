use crate::freeze_watchdog::force_kill_pid;
use crate::system::fetch_live_processes_win32;
use std::process::Command;

pub fn handle_cli_args(args: Vec<String>) -> bool {
    // If no subcommands provided (just `cargo run`), return false to launch TUI dashboard
    if args.len() < 2 {
        return false;
    }

    let subcommand = args[1].to_lowercase();

    match subcommand.as_str() {
        "status" => {
            print_quick_status();
            true
        }
        "kill" => {
            if args.len() < 3 {
                println!("❌ Usage: cargo run -- kill <process_name_or_pid>");
            } else {
                fuzzy_kill_process(&args[2]);
            }
            true
        }
        "service" => {
            if args.len() < 4 {
                println!("❌ Usage: cargo run -- service <enable|disable|manual> <service_name>");
            } else {
                manage_windows_service(&args[2], &args[3]);
            }
            true
        }
        "install" => {
            if args.len() < 3 {
                println!("❌ Usage: cargo run -- install <package_name>");
            } else {
                install_package_winget(&args[2]);
            }
            true
        }
        "--help" | "-h" => {
            print_help_menu();
            true
        }
        _ => false, // Unknown subcommand -> launch main dashboard
    }
}

// Quick system health snapshot

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

// instant fuzzy kill command (Matches Process Name or PID)

fn fuzzy_kill_process(target: &str) {
    println!("Searching for process matching '{}'...", target);

    //If target is a raw PID

    if let Ok(pid) = target.parse::<u32>() {
        if force_kill_pid(pid) {
            println!("Successfully killed PID : {}", pid);
        } else {
            println!("Failed to kill PID {}, Check process permission.", pid);
        }
        return;
    }

    //Otherwise, match against running process names

    if let Ok(processes) = fetch_live_processes_win32() {
        let matches: Vec<_> = processes
            .into_iter()
            .filter(|p| p.name.to_lowercase().contains(&target.to_lowercase()))
            .collect();
        if matches.is_empty() {
            println!("No running processes found matching '{}'.", target);
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

// Linux-style Windows service control manager interop (sc.exe)

fn manage_windows_service(action: &str, service_name: &str) {
    let start_type = match action {
        "disable" => "disabled",
        "enable" => "auto",
        "manual" => "demand",
        _ => {
            println!(
                "❌ Invalid action '{}'. Use: enable, disable, or manual.",
                action
            );
            return;
        }
    };

    println!(
        "⚙️ Configuring Windows Service '{}' to start={}...",
        service_name, start_type
    );

    // Call sc.exe with arguments
    let output = Command::new("sc")
        .args(["config", service_name, &format!("start={}", start_type)])
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);

            if out.status.success() {
                println!(
                    "✅ Service '{}' successfully updated to start={}.",
                    service_name, start_type
                );
                if !stdout.is_empty() {
                    println!("{}", stdout.trim());
                }
            } else {
                println!("❌ Error configuring service:");
                if !stderr.is_empty() {
                    println!("{}", stderr.trim());
                } else {
                    println!("{}", stdout.trim());
                }
                println!(
                    "💡 Tip: Modifying Windows services requires opening Terminal / PowerShell as Administrator!"
                );
            }
        }
        Err(e) => println!("❌ Failed to execute Win32 Service Manager (sc.exe): {}", e),
    }
}

// Native Windows Package Manager (winget) wrapper

fn install_package_winget(package_name: &str) {
    println!(
        "📦 Searching & installing package '{}' via Winget...",
        package_name
    );

    let mut child = Command::new("winget")
        .args([
            "install",
            "--id",
            package_name,
            "--silent",
            "--accept-source-agreements",
            "--accept-package-agreements",
        ])
        .spawn();

    match child {
        Ok(mut process) => {
            let status = process.wait();
            if let Ok(s) = status {
                if s.success() {
                    println!("🎉 Package '{}' successfully installed!", package_name);
                } else {
                    println!("⚠️ Winget installation finished with non-zero exit code.");
                }
            }
        }
        Err(_) => println!("❌ Winget package manager not found on this system."),
    }
}

fn print_help_menu() {
    println!("==========================================================================");
    println!("   🛡️  SENTINEL CLI HELP MENU");
    println!("==========================================================================");
    println!("Usage: sentinel [subcommand] [options]\n");
    println!("Commands:");
    println!("  status                        Displays a quick system health snapshot");
    println!("  kill <name|pid>               Fuzzy kills matching active processes");
    println!("  service <enable|disable> <name> Toggles background Windows Services");
    println!("  install <package_id>          Silent installation via Winget");
    println!("  --help, -h                    Displays this help menu\n");
    println!("If no subcommands are passed, launches the interactive live TUI dashboard.");
}
