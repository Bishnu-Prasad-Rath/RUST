use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct CleanTarget {
    pub name: String,
    pub path: PathBuf,
    pub size_bytes: u64,
}

pub fn run_interactive_cleaner() {
    println!("🔍 Scanning system for cache, temp files, and junk...\n");

    let mut targets = get_cleaning_targets();

    loop {
        if targets.is_empty() {
            println!("✨ Nothing left in the hit list to clean.");
            return;
        }

        let total_potential_bytes: u64 = targets.iter().map(|t| t.size_bytes).sum();

        // 1. The "Dry Run" - Display folders with human-readable sizes
        println!("🗑️  THE CLEANING HIT LIST:");
        println!("--------------------------------------------------------------------------");
        for (i, target) in targets.iter().enumerate() {
            println!(
                "[{}] {:<22} | {:>10} | {}",
                i,
                target.name,
                format_bytes(target.size_bytes),
                target.path.display()
            );
        }
        println!("--------------------------------------------------------------------------");
        println!(
            "📊 Total Potential Space to Reclaim: {}",
            format_bytes(total_potential_bytes)
        );
        println!("--------------------------------------------------------------------------");

        // 2. The Interactive Prompt
        print!("\nDo you want to nuke all of these? (y = Yes / n = Exclude folder / q = Quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_lowercase();

        match choice.as_str() {
            "y" | "yes" => {
                execute_cleanup(&targets);
                break;
            }
            "n" | "no" => {
                targets = handle_exclusions(targets);
            }
            "q" | "quit" => {
                println!("🛑 Aborting clean operation.");
                break;
            }
            _ => {
                println!("❌ Invalid input. Please type 'y', 'n', or 'q'.\n");
            }
        }
    }
}

/// Discovers targets and calculates their current sizes
fn get_cleaning_targets() -> Vec<CleanTarget> {
    let mut candidate_paths: Vec<(String, PathBuf)> = Vec::new();

    // User Temp (%TEMP%)
    if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
        candidate_paths.push((
            "Windows User Temp".to_string(),
            PathBuf::from(&local_app_data).join("Temp"),
        ));

        // NPM Cache
        candidate_paths.push((
            "NPM Global Cache".to_string(),
            PathBuf::from(&local_app_data).join("npm-cache"),
        ));

        // Crash Dumps
        candidate_paths.push((
            "Crash Dumps".to_string(),
            PathBuf::from(&local_app_data).join("CrashDumps"),
        ));
    }

    // System Temp (C:\Windows\Temp)
    if let Ok(windir) = env::var("WINDIR") {
        candidate_paths.push((
            "Windows System Temp".to_string(),
            PathBuf::from(&windir).join("Temp"),
        ));
    }

    // Build the targets list with calculated sizes
    let mut targets = Vec::new();
    for (name, path) in candidate_paths {
        if path.exists() {
            let size = calculate_dir_size(&path);
            targets.push(CleanTarget {
                name,
                path,
                size_bytes: size,
            });
        }
    }

    targets
}

/// Recursively calculates the byte size of any folder
fn calculate_dir_size(path: &Path) -> u64 {
    let mut total_size = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_file() {
                if let Ok(metadata) = entry.metadata() {
                    total_size += metadata.len();
                }
            } else if entry_path.is_dir() {
                total_size += calculate_dir_size(&entry_path);
            }
        }
    }

    total_size
}

/// Interactive exclusion handler
fn handle_exclusions(mut targets: Vec<CleanTarget>) -> Vec<CleanTarget> {
    print!("Enter the number [0, 1, 2...] of the folder to EXCLUDE: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(index) = input.trim().parse::<usize>() {
        if index < targets.len() {
            let removed = targets.remove(index);
            println!(
                "🛡️  Excluded: {} ({})",
                removed.name,
                removed.path.display()
            );
            println!("Refreshing Hit List...\n");
        } else {
            println!("❌ Invalid number. Out of range.\n");
        }
    } else {
        println!("❌ Please enter a valid number.\n");
    }

    targets
}

/// Recursively deletes folder contents, skips in-use locked files, and tracks freed bytes
fn execute_cleanup(targets: &[CleanTarget]) {
    println!("\n🔥 INITIATING PURGE...");

    let mut total_freed_bytes: u64 = 0;
    let mut total_files_removed: u64 = 0;

    for target in targets {
        print!("🗑️  Cleaning {:<20} ... ", target.name);
        io::stdout().flush().unwrap();

        let (freed, files) = wipe_dir_contents(&target.path);
        total_freed_bytes += freed;
        total_files_removed += files;

        println!("Freed: {}", format_bytes(freed));
    }

    println!("\n==========================================================================");
    println!("   ✨ CLEANUP SUMMARY");
    println!("==========================================================================");
    println!("📁 Files Removed     : {}", total_files_removed);
    println!(
        "💾 Total Disk Space  : {} Reclaimed",
        format_bytes(total_freed_bytes)
    );
    println!("==========================================================================\n");
}

/// Safely deletes files inside a folder without deleting the root target directory itself
fn wipe_dir_contents(path: &Path) -> (u64, u64) {
    let mut freed_bytes = 0;
    let mut files_deleted = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_file() {
                if let Ok(meta) = entry.metadata() {
                    let len = meta.len();
                    // Attempt deletion (skips silently if file is locked by a running process)
                    if fs::remove_file(&entry_path).is_ok() {
                        freed_bytes += len;
                        files_deleted += 1;
                    }
                }
            } else if entry_path.is_dir() {
                let (sub_freed, sub_files) = wipe_dir_contents(&entry_path);
                freed_bytes += sub_freed;
                files_deleted += sub_files;
                // Attempt to remove empty directory after clearing its contents
                let _ = fs::remove_dir(&entry_path);
            }
        }
    }

    (freed_bytes, files_deleted)
}

/// Helper to convert raw bytes into human-friendly strings (KB, MB, GB)
fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let b = bytes as f64;
    if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= MB {
        format!("{:.2} MB", b / MB)
    } else if b >= KB {
        format!("{:.2} KB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}
