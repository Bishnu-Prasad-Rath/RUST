use crate::cli::stage::trigger_event_in_directory;
use crate::engine::process::force_kill_pid;
use std::collections::{HashMap, HashSet};
use std::thread;
use std::time::{Duration, Instant};

use windows_sys::Win32::Foundation::{BOOL, HWND};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, MOD_ALT, MOD_CONTROL, MOD_SHIFT, VK_ESCAPE,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetForegroundWindow, GetMessageW, GetWindowTextLengthW, GetWindowTextW,
    GetWindowThreadProcessId, IsHungAppWindow, IsWindowVisible, MSG, WM_HOTKEY,
};

pub struct FreezeTracker {
    frozen_since: HashMap<u32, Instant>,
    // In-memory set of all workspaces already triggered in this Sentinel daemon session
    triggered_workspaces: HashSet<String>,
}

impl FreezeTracker {
    pub fn new() -> Self {
        Self {
            frozen_since: HashMap::new(),
            triggered_workspaces: HashSet::new(),
        }
    }

    pub fn scan_and_enforce(&mut self, threshold_secs: u64) -> Vec<u32> {
        let current_frozen = get_all_hung_pids();
        let mut terminated_pids = Vec::new();

        self.frozen_since
            .retain(|pid, _| current_frozen.contains(pid));

        for pid in current_frozen {
            let entry = self.frozen_since.entry(pid).or_insert_with(Instant::now);

            if entry.elapsed() >= Duration::from_secs(threshold_secs) {
                println!(
                    "\n🚨 [WATCHDOG ALERT] PID {} unresponsive for > {}s! Force Terminating...",
                    pid, threshold_secs
                );
                if force_kill_pid(pid) {
                    terminated_pids.push(pid);
                }
            }
        }
        terminated_pids
    }

    /// Single-Trigger Workspace Sentinel: Fires exactly once per unique project workspace per daemon lifecycle
    pub fn poll_active_ide_workspace(&mut self) {
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd == 0 {
                return;
            }

            let len = GetWindowTextLengthW(hwnd);
            if len <= 0 {
                return;
            }

            let mut buffer: Vec<u16> = vec![0; (len + 1) as usize];
            let read_len = GetWindowTextW(hwnd, buffer.as_mut_ptr(), len + 1);

            if read_len > 0 {
                let raw_title = String::from_utf16_lossy(&buffer[..read_len as usize]);

                // Only evaluate if it's VS Code
                if raw_title.contains("Visual Studio Code") {
                    if let Some(detected_workspace) = parse_vscode_workspace_name(&raw_title) {
                        let normalized_key = detected_workspace.to_lowercase();

                        // 🔒 STRICT CHECK: If already triggered this session, ignore completely (Fixes Alt+Tab bug)
                        if self.triggered_workspaces.contains(&normalized_key) {
                            return;
                        }

                        // Verify if a sentinel.yml actually exists in a folder matching this workspace name
                        // Or use your current directory check logic:
                        println!(
                            "\n👁️  [SENTINEL DAEMON] Initial project activation for: '{}'",
                            detected_workspace
                        );

                        // Lock it immediately so Alt+Tab or refocused windows can never trigger it again
                        self.triggered_workspaces.insert(normalized_key);

                        let workspace_name = detected_workspace.clone();
                        thread::spawn(move || {
                            trigger_event_in_directory("open", &workspace_name);
                        });
                    }
                }
            }
        }
    }

    pub fn cleanup_closed_workspaces(&mut self) {
        let all_titles = get_all_window_titles();
        let mut currently_open_workspaces = HashSet::new();

        // 1. Find every VS Code window currently open on the computer
        for title in all_titles {
            if title.contains("Visual Studio Code") {
                if let Some(ws) = parse_vscode_workspace_name(&title) {
                    currently_open_workspaces.insert(ws.to_lowercase());
                }
            }
        }

        // 2. The "Diff": Remove workspaces from memory if they are no longer open
        // retain() keeps only the items that return true.
        self.triggered_workspaces.retain(|locked_ws| {
            let is_still_open = currently_open_workspaces.contains(locked_ws);
            if !is_still_open {
                println!(
                    "\n🧹 [DAEMON] Workspace Closed. Unlocking '{}' for future triggers.",
                    locked_ws
                );
            }
            is_still_open
        });
    }
}

/// Win32 API to fetch every single window title on the desktop
fn get_all_window_titles() -> Vec<String> {
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
                        (*titles_ptr).push(title);
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

/// Parses the active workspace name reliably
fn parse_vscode_workspace_name(title: &str) -> Option<String> {
    let cleaned_title = title.replace('●', "");
    let parts: Vec<&str> = cleaned_title.split(" - ").map(|s| s.trim()).collect();

    if parts.len() < 2 {
        return None;
    }

    let candidate = if parts.len() >= 3 {
        parts[parts.len() - 2]
    } else {
        parts[0]
    };

    if candidate.is_empty()
        || candidate == "Visual Studio Code"
        || candidate.eq_ignore_ascii_case("Welcome")
        || candidate.eq_ignore_ascii_case("Settings")
        || candidate.eq_ignore_ascii_case("Getting Started")
    {
        return None;
    }

    Some(candidate.to_string())
}

fn get_all_hung_pids() -> Vec<u32> {
    let mut hung_pids: Vec<u32> = Vec::new();

    unsafe {
        unsafe extern "system" fn enum_window_callback(hwnd: HWND, lparam: isize) -> BOOL {
            if lparam == 0 {
                return 0;
            }
            let pids = &mut *(lparam as *mut Vec<u32>);

            if IsWindowVisible(hwnd) != 0 && IsHungAppWindow(hwnd) != 0 {
                let mut pid: u32 = 0;
                GetWindowThreadProcessId(hwnd, &mut pid);

                if pid > 0 && !pids.contains(&pid) {
                    pids.push(pid);
                }
            }
            1
        }
        EnumWindows(
            Some(enum_window_callback),
            &mut hung_pids as *mut Vec<u32> as isize,
        );
    }
    hung_pids
}

pub fn emergency_kill_foreground_app() {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd != 0 {
            let mut pid: u32 = 0;
            GetWindowThreadProcessId(hwnd, &mut pid);

            if pid > 0 {
                println!(
                    "\n⚡ [EMERGENCY PANIC HOTKEY TRIGGERED] Terminating foreground PID: {}",
                    pid
                );
                force_kill_pid(pid);
            }
        }
    }
}

pub fn start_global_hotkey_listener() {
    thread::spawn(|| unsafe {
        let hotkey_id = 1001;
        let modifiers = MOD_CONTROL | MOD_ALT | MOD_SHIFT;

        if RegisterHotKey(0, hotkey_id, modifiers, VK_ESCAPE as u32) == 0 {
            println!("⚠️ Failed to register global panic Hotkey (Ctrl + Alt + Shift + Esc)");
            return;
        }

        println!("⚡ Global Panic Hotkey Registered: [Ctrl + Alt + Shift + Esc]");

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, 0, 0, 0) > 0 {
            if msg.message == WM_HOTKEY && msg.wParam == hotkey_id as usize {
                emergency_kill_foreground_app();
            }
        }

        UnregisterHotKey(0, hotkey_id);
    });
}
