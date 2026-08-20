use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

// --- WIN32 IMPORTS FOR WINDOW SCANNING ---
use sysinfo::System;
use windows_sys::Win32::Foundation::{BOOL, HWND};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible,
};

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Serialize, Deserialize)]
pub struct SentinelStageConfig {
    pub project_name: Option<String>,
    pub browser_profile: Option<String>,
    pub events: Option<HashMap<String, String>>,
    pub stages: HashMap<String, StageDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StageDefinition {
    pub description: Option<String>,
    pub web_links: Option<Vec<String>>,
    pub daemons: Option<Vec<String>>,
    pub tasks: Option<Vec<String>>,
}

pub fn trigger_event_in_directory(event_name: &str, workspace_hint: &str) {
    let target_yml = find_yaml_for_workspace(workspace_hint);

    if let Some(path) = target_yml {
        if let Ok(file) = File::open(&path) {
            if let Ok(config) = serde_yaml::from_reader::<_, SentinelStageConfig>(file) {
                let event_key = format!("on_{}", event_name.trim_start_matches("on_"));
                if let Some(ref event_map) = config.events {
                    if let Some(target_stage) = event_map.get(&event_key) {
                        println!(
                            "⚡ [DAEMON AUTO-TRIGGER] Executing '{}' for workspace '{}'",
                            target_stage, workspace_hint
                        );
                        execute_stage_with_config(&config, target_stage, true);
                    }
                }
            }
        }
    }
}

pub fn trigger_event(event_name: &str, detach: bool) {
    let yml_file = match find_sentinel_yaml() {
        Some(p) => p,
        None => {
            println!("❌ No 'sentinel.yml' found in the current directory.");
            return;
        }
    };

    let file = match File::open(&yml_file) {
        Ok(f) => f,
        Err(e) => {
            println!("❌ Failed to read '{}': {}", yml_file, e);
            return;
        }
    };

    let config: SentinelStageConfig = match serde_yaml::from_reader(file) {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("❌ YAML Parsing Error: {}", e);
            return;
        }
    };

    let event_key = format!("on_{}", event_name.trim_start_matches("on_"));

    if let Some(ref event_map) = config.events {
        if let Some(target_stage) = event_map.get(&event_key) {
            println!(
                "⚡ [EVENT DETECTED] '{}' mapped to Stage '{}'",
                event_key, target_stage
            );
            execute_stage_with_config(&config, target_stage, detach);
            return;
        }
    }
    println!(
        "⚠️ No stage mapped to event '{}' in sentinel.yml",
        event_key
    );
}

pub fn execute_project_stage(stage_name: &str, detach: bool) {
    let yml_file = match find_sentinel_yaml() {
        Some(p) => p,
        None => {
            println!("❌ No 'sentinel.yml' found in the current directory.");
            println!("💡 Run 'cargo run -- stage init' to generate one.");
            return;
        }
    };
    let file = match File::open(&yml_file) {
        Ok(f) => f,
        Err(_) => return,
    };
    let config: SentinelStageConfig = match serde_yaml::from_reader(file) {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("❌ Failed to parse 'sentinel.yml'. Check syntax: {}", e);
            return;
        }
    };

    execute_stage_with_config(&config, stage_name, detach);
}

fn execute_stage_with_config(config: &SentinelStageConfig, stage_name: &str, detach: bool) {
    let project_label = config
        .project_name
        .clone()
        .unwrap_or_else(|| "Active Project".to_string());
    let target_profile = config
        .browser_profile
        .clone()
        .unwrap_or_else(|| "Default".to_string());

    let stage = match config.stages.get(stage_name) {
        Some(s) => s,
        None => {
            println!("❌ Stage '{}' not defined in sentinel.yml", stage_name);
            return;
        }
    };

    println!("==========================================================================");
    println!(
        "   🚀 EXECUTING STAGE: [{}] -> {}",
        project_label,
        stage_name.to_uppercase()
    );
    if let Some(ref desc) = stage.description {
        println!("   📝 Description: {}", desc);
    }
    println!("==========================================================================");

    // 1. Spawning Web Links in Smart App Mode
    if let Some(ref links) = stage.web_links {
        if !links.is_empty() {
            let resolved_urls: Vec<String> = links
                .iter()
                .map(|url| normalize_authenticated_portal_url(url))
                .collect();

            println!("\n🌐 Launching Authenticated Sessions...");
            launch_urls_smart_app_mode(&resolved_urls, &target_profile);
        }
    }

    // 2. Starting Background Daemons
    if let Some(ref daemons) = stage.daemons {
        println!("\n🐳 Spawning Background Daemon(s)...");
        for daemon in daemons {
            println!("   └─ Executing: {}", daemon);
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", daemon]);
            cmd.creation_flags(CREATE_NO_WINDOW);
            cmd.stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());
            let _ = cmd.spawn();
        }
    }

    // 3. Spawning Tasks
    if let Some(ref tasks) = stage.tasks {
        println!("\n⚡ Spawning Pipeline Task(s)...");
        for task in tasks {
            println!("   └─ Running: {}", task);
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", task]);
            if detach {
                cmd.creation_flags(CREATE_NO_WINDOW);
                cmd.stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null());
                let _ = cmd.spawn();
            } else {
                let _ = cmd.spawn();
            }
        }
    }

    println!("\n✨ Pipeline triggered successfully!");
}

fn normalize_authenticated_portal_url(url: &str) -> String {
    let clean = url.trim();
    if clean == "https://cloud.mongodb.com" || clean == "https://cloud.mongodb.com/" {
        return "https://cloud.mongodb.com/v2".to_string();
    }
    if clean == "https://aws.amazon.com" {
        return "https://us-east-1.console.aws.amazon.com/console/home".to_string();
    }
    if clean == "https://supabase.com" {
        return "https://supabase.com/dashboard/projects".to_string();
    }
    clean.to_string()
}

// --- INTELLIGENT UX LOGIC ---

fn extract_domain_keyword(url: &str) -> String {
    let clean_url = url
        .trim_start_matches("http://")
        .trim_start_matches("https://");
    let domain_port = clean_url.split('/').next().unwrap_or(clean_url);
    let domain = domain_port.split(':').next().unwrap_or(domain_port);

    if domain.contains("mail.google.com") || domain.contains("gmail") {
        return "gmail".to_string();
    }
    if domain.contains("drive.google.com") {
        return "google drive".to_string();
    }
    if domain.contains("cloud.mongodb.com") {
        return "mongodb".to_string();
    }
    if domain.contains("aws.amazon.com") {
        return "aws".to_string();
    }
    if domain.contains("localhost") || domain.starts_with("127.0.0.1") {
        return "localhost".to_string();
    }

    let parts: Vec<&str> = domain.split('.').collect();
    if parts.len() <= 1 {
        return parts[0].to_lowercase();
    }
    parts[parts.len() - 2].to_lowercase()
}

fn get_all_desktop_window_titles() -> Vec<String> {
    let mut titles: Vec<String> = Vec::new();
    unsafe {
        unsafe extern "system" fn enum_window_callback(hwnd: HWND, lparam: isize) -> BOOL {
            if IsWindowVisible(hwnd) != 0 {
                let len = GetWindowTextLengthW(hwnd);
                if len > 0 {
                    let mut buffer: Vec<u16> = vec![0; (len + 1) as usize];
                    let read_len = GetWindowTextW(hwnd, buffer.as_mut_ptr(), len + 1);
                    if read_len > 0 {
                        let title = String::from_utf16_lossy(&buffer[..read_len as usize]);
                        let titles_ptr = lparam as *mut Vec<String>;
                        (*titles_ptr).push(title.to_lowercase());
                    }
                }
            }
            1
        }
        EnumWindows(
            Some(enum_window_callback),
            &mut titles as *mut Vec<String> as isize,
        );
    }
    titles
}

fn find_browser_executable() -> Option<PathBuf> {
    let prog_files = env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".to_string());
    let prog_files_x86 =
        env::var("ProgramFiles(x86)").unwrap_or_else(|_| r"C:\Program Files (x86)".to_string());
    let local_app_data = env::var("LOCALAPPDATA").unwrap_or_default();

    let mut candidate_paths = vec![
        Path::new(&prog_files).join(r"Microsoft\Edge\Application\msedge.exe"),
        Path::new(&prog_files_x86).join(r"Microsoft\Edge\Application\msedge.exe"),
        Path::new(&prog_files).join(r"Google\Chrome\Application\chrome.exe"),
        Path::new(&prog_files_x86).join(r"Google\Chrome\Application\chrome.exe"),
    ];
    if !local_app_data.is_empty() {
        candidate_paths
            .push(Path::new(&local_app_data).join(r"Google\Chrome\Application\chrome.exe"));
    }

    candidate_paths.into_iter().find(|p| p.exists())
}

fn launch_urls_smart_app_mode(urls: &[String], profile_name: &str) {
    let browser_exe = find_browser_executable();
    let open_titles = get_all_desktop_window_titles();

    for url in urls {
        let keyword = extract_domain_keyword(url);

        let is_already_open = open_titles.iter().any(|title| {
            let clean_title = title
                .replace(" - google chrome", "")
                .replace(" - microsoft edge", "")
                .replace(" - brave", "")
                .replace(" - mozilla firefox", "");

            clean_title.contains(&keyword)
        });

        if is_already_open {
            println!(
                "   ✅ [{}] is already running. Skipping duplicate.",
                keyword
            );
            continue;
        }

        println!("   🚀 Launching [{}] in App Mode...", keyword);

        if let Some(ref exe) = browser_exe {
            let arg_app = format!("--app={}", url);
            let arg_profile = format!("--profile-directory={}", profile_name);

            let _ = Command::new(exe)
                .args([&arg_app, &arg_profile])
                .creation_flags(CREATE_NO_WINDOW)
                .spawn();
        } else {
            let _ = Command::new("explorer")
                .arg(url)
                .creation_flags(CREATE_NO_WINDOW)
                .spawn();
        }
    }
}

pub fn init_starter_yaml() {
    let yaml_path = Path::new("sentinel.yml");

    if yaml_path.exists() {
        println!("⚠️  A 'sentinel.yml' file already exists in this directory.");
        return;
    }

    let starter_template = r#"project_name: "YT-NEO-Env"
browser_profile: "Default"

stages:
  dev:
    description: "Boots MERN stack, Redis, and WebSockets environment"
    daemons:
      - "redis-server"
    tasks:
      - "npm run server"
      - "npm run client"
    web_links:
      - "http://localhost:3000"
      - "https://cloud.mongodb.com"
      - "code ."

  build:
    description: "Builds production bundles"
    tasks:
      - "npm run build"

events:
  on_open: "dev"
  on_git_push: "build"
"#;

    match fs::write(yaml_path, starter_template) {
        Ok(_) => {
            println!("✅ Initialized starter 'sentinel.yml' successfully!");
            println!("💡 Customize it for your project, then run: cargo run -- stage dev");
        }
        Err(e) => {
            println!("❌ Failed to create 'sentinel.yml': {}", e);
        }
    }
}

fn find_sentinel_yaml() -> Option<String> {
    if Path::new("sentinel.yml").exists() {
        Some("sentinel.yml".to_string())
    } else {
        None
    }
}

fn find_yaml_for_workspace(workspace_hint: &str) -> Option<PathBuf> {
    let hint_lower = workspace_hint.to_lowercase();
    println!("🔍 [DAEMON] Searching for workspace: '{}'", workspace_hint);

    // STRATEGY 1: Smart Sibling Scan (Bypasses Windows Security Blocks)
    // If running in D:\RUST\sentinel-pkg, it scans D:\RUST for the target project
    if let Ok(cwd) = std::env::current_dir() {
        // Check if the current directory is the target
        if let Some(name) = cwd.file_name().and_then(|n| n.to_str()) {
            if name.to_lowercase() == hint_lower {
                let local = cwd.join("sentinel.yml");
                if local.exists() {
                    println!("   ✅ Found matching path in Current Directory: {:?}", cwd);
                    return Some(local);
                }
            }
        }

        // Check sibling directories in the same parent folder
        if let Some(parent) = cwd.parent() {
            if let Ok(entries) = std::fs::read_dir(parent) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            if let Some(folder_name) = entry.file_name().to_str() {
                                if folder_name.to_lowercase() == hint_lower {
                                    let matched_dir = entry.path();
                                    let yaml_path = matched_dir.join("sentinel.yml");

                                    if yaml_path.exists() {
                                        println!("   ✅ Found matching path via Smart Sibling Scan: {:?}", matched_dir);
                                        // ⚡ CRITICAL: Change directory so npm/cargo commands run in the right place
                                        let _ = std::env::set_current_dir(&matched_dir);
                                        return Some(yaml_path);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // STRATEGY 2: Sysinfo Fallback (For when VS Code is on a completely different drive)
    let mut sys = sysinfo::System::new_all();
    sys.refresh_processes();

    for (_pid, process) in sys.processes() {
        if process.name().to_lowercase().contains("code") {
            if let Some(cwd) = process.cwd() {
                if let Some(folder_name) = cwd.file_name().and_then(|n| n.to_str()) {
                    if folder_name.to_lowercase() == hint_lower {
                        let yaml_path = cwd.join("sentinel.yml");
                        if yaml_path.exists() {
                            println!("   ✅ Found matching path via sysinfo CWD: {:?}", cwd);
                            let _ = std::env::set_current_dir(cwd);
                            return Some(yaml_path);
                        }
                    }
                }
            }

            for arg in process.cmd() {
                let target_dir = PathBuf::from(arg);
                if target_dir.is_absolute() && target_dir.is_dir() {
                    if let Some(folder_name) = target_dir.file_name().and_then(|n| n.to_str()) {
                        if folder_name.to_lowercase() == hint_lower {
                            let yaml_path = target_dir.join("sentinel.yml");
                            if yaml_path.exists() {
                                println!(
                                    "   ✅ Found matching path via sysinfo CMD: {:?}",
                                    target_dir
                                );
                                let _ = std::env::set_current_dir(&target_dir);
                                return Some(yaml_path);
                            }
                        }
                    }
                }
            }
        }
    }

    println!(
        "❌ [DAEMON] Could not locate '{}' with a valid sentinel.yml",
        workspace_hint
    );
    None
}
