use crate::engine::process::{fetch_live_processes_win32, get_process_exe_path};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, SystemTime};

pub fn save_workspace_snapshot(snapshot_name: &str) {
    clean_expired_snapshots();
    println!("📸 Capturing workspace snapshot '{}'...", snapshot_name);

    if let Ok(processes) = fetch_live_processes_win32() {
        let mut paths = Vec::new();

        for proc in processes {
            if proc.memory_mb > 15 {
                if let Some(path) = get_process_exe_path(proc.pid) {
                    let p_lower = path.to_lowercase();
                    if !p_lower.contains("windows\\system32") && !p_lower.contains("svchost") {
                        if !paths.contains(&path) {
                            paths.push(path);
                        }
                    }
                }
            }
        }

        let dir = get_snapshot_dir();
        let _ = fs::create_dir_all(&dir);
        let file_path = dir.join(format!("{}.json", snapshot_name));

        if let Ok(json) = serde_json::to_string_pretty(&paths) {
            if let Ok(mut f) = File::create(&file_path) {
                let _ = f.write_all(json.as_bytes());
                println!("✅ Snapshot saved! Apps tracked: {}", paths.len());
            }
        }
    }
}

pub fn restore_workspace_snapshot(snapshot_name: &str) {
    clean_expired_snapshots();
    let file_path = get_snapshot_dir().join(format!("{}.json", snapshot_name));

    if !file_path.exists() {
        println!("❌ Snapshot '{}' not found or expired.", snapshot_name);
        return;
    }

    if let Ok(mut f) = File::open(&file_path) {
        let mut data = String::new();
        if f.read_to_string(&mut data).is_ok() {
            if let Ok(paths) = serde_json::from_str::<Vec<String>>(&data) {
                println!("🔄 Restoring {} application(s)...", paths.len());
                for path in paths {
                    println!("   └─ Launching: {}", path);
                    let _ = Command::new(&path).spawn();
                }
                println!("✨ Workspace restored successfully!");
            }
        }
    }
}

pub fn clean_expired_snapshots() {
    let dir = get_snapshot_dir();
    if !dir.exists() {
        return;
    }

    let max_age = Duration::from_secs(7 * 24 * 60 * 60);
    let now = SystemTime::now();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                if let Ok(modified) = meta.modified() {
                    if let Ok(age) = now.duration_since(modified) {
                        if age > max_age {
                            let _ = fs::remove_file(entry.path());
                            println!("🧹 Auto-purged expired snapshot: {:?}", entry.file_name());
                        }
                    }
                }
            }
        }
    }
}

fn get_snapshot_dir() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap_or_default();
    dir.push(".sentinel_snapshots");
    dir
}
