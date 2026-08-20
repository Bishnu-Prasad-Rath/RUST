use crate::engine::process::{force_kill_pid, log_event};
use crate::models::{HealthStatus, ProcessCategory, ProcessInfo, ThreatLevel};
use std::io::{self, Write};

pub fn scan_process_threats(proc: &mut ProcessInfo) {
    let name_lower = proc.name.to_lowercase();

    let suspicious_keywords = [
        "mimikatz",
        "keylogger",
        "netcat",
        "psexec",
        "hacktool",
        "miner",
    ];

    for keyword in suspicious_keywords {
        if name_lower.contains(keyword) {
            let reason = format!("Known suspicious keyword detected: '{}'", keyword);
            proc.status = HealthStatus::SecurityRisk(ThreatLevel::Critical(reason));
            return;
        }
    }

    if name_lower == "powershell.exe" || name_lower == "cmd.exe" || name_lower == "wscript.exe" {
        if proc.memory_mb > 300 {
            let reason = format!(
                "High-memory script execution environment ({}MB)",
                proc.memory_mb
            );
            proc.status = HealthStatus::SecurityRisk(ThreatLevel::Medium(reason));
        }
    }
}

pub fn enforce_security_and_limits(processes: &mut [ProcessInfo], memory_threshold_mb: u64) {
    for proc in processes.iter_mut() {
        scan_process_threats(proc);

        if let HealthStatus::SecurityRisk(ref threat) = proc.status {
            match threat {
                ThreatLevel::Critical(reason) | ThreatLevel::Medium(reason) => {
                    println!("\n🚨 MINI DEFENDER ALERT: Threat detected!");
                    println!(
                        "   └─ Name: {} | PID: {} | Reason: {}",
                        proc.name, proc.pid, reason
                    );

                    let prompt = format!(
                        "   └─ Do you want to terminate security threat PID {} ({})?",
                        proc.pid, proc.name
                    );
                    if ask_user_confirmation(&prompt) {
                        if force_kill_pid(proc.pid) {
                            proc.status = HealthStatus::Terminated;
                            log_event(
                                "TERMINATED",
                                &format!(
                                    "Killed security threat {} (PID: {})",
                                    proc.name, proc.pid
                                ),
                            );
                        }
                    }
                }
                _ => {}
            }
            continue;
        }

        if proc.category == ProcessCategory::UserApp {
            if let HealthStatus::HighMemoryUsage(mem) = proc.status {
                if mem >= memory_threshold_mb {
                    println!("\n🚨 ALERT: High memory usage detected on User App!");
                    println!(
                        "   └─ Name: {} | PID: {} | Memory: {} MB (Limit: {} MB)",
                        proc.name, proc.pid, mem, memory_threshold_mb
                    );

                    let prompt = format!(
                        "   └─ Do you want to terminate PID {} ({})?",
                        proc.pid, proc.name
                    );
                    if ask_user_confirmation(&prompt) {
                        if force_kill_pid(proc.pid) {
                            proc.status = HealthStatus::Terminated;
                            log_event(
                                "TERMINATED",
                                &format!(
                                    "Killed high-memory app {} (PID: {})",
                                    proc.name, proc.pid
                                ),
                            );
                        }
                    }
                }
            }
        }
    }
}

fn ask_user_confirmation(prompt: &str) -> bool {
    print!("{} (Y/N): ", prompt);
    let _ = io::stdout().flush();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let cleaned = input.trim();
        return cleaned.eq_ignore_ascii_case("y") || cleaned.eq_ignore_ascii_case("yes");
    }
    false
}
