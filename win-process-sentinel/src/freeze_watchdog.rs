use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};
use windows_sys::Win32::Foundation::CloseHandle;
use windows_sys::Win32::Foundation::{BOOL, HWND};
use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_TERMINATE, TerminateProcess};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    MOD_ALT, MOD_CONTROL, MOD_SHIFT, RegisterHotKey, UnregisterHotKey, VK_ESCAPE,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetForegroundWindow, GetWindowThreadProcessId, IsHungAppWindow, IsWindowVisible,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY};

// Needed Struct to track how long a PID has been unresponsive

pub struct FreezeTracker {
    frozen_since: HashMap<u32, Instant>,
}

impl FreezeTracker {
    pub fn new() -> Self {
        Self {
            frozen_since: HashMap::new(),
        }
    }

    //Scans top-level windows and returns PIDS that have beenfrozen for > threshold_secs

    pub fn scan_and_enforce(&mut self, threshold_secs: u64) -> Vec<u32> {
        let current_frozen = get_all_hung_pids();
        let mut terminated_pids = Vec::new();

        // 1. Remove PIDs from map that are no longer frozen

        self.frozen_since
            .retain(|pid, _| current_frozen.contains(pid));

        // 2. Track new frozen PIDs or enforce timeout

        for pid in current_frozen {
            let entry = self.frozen_since.entry(pid).or_insert_with(Instant::now);

            if entry.elapsed() >= Duration::from_secs(threshold_secs) {
                println!(
                    "\n [WATCH-DOG ALERT] PID {} unresponsive for > {}s! Force Terminating...",
                    pid, threshold_secs
                );
                if force_kill_pid(pid) {
                    terminated_pids.push(pid);
                }
            }
        }
        terminated_pids
    }
}

// For itertating active top-level windows using Win32 EnumWindows

fn get_all_hung_pids() -> Vec<u32> {
    let mut hung_pids = Vec::new();

    unsafe {
        unsafe extern "system" fn enum_window_callback(hwnd: HWND, lparam: isize) -> BOOL {
            let hung_pids_ptr = lparam as *mut Vec<u32>;

            if IsWindowVisible(hwnd) != 0 && IsHungAppWindow(hwnd) != 0 {
                let mut pid: u32 = 0;

                GetWindowThreadProcessId(hwnd, &mut pid);

                if pid > 0 {
                    let pids = &mut *hung_pids_ptr;
                    if !pids.contains(&pid) {
                        pids.push(pid);
                    }
                }
            }
            1
        }
        EnumWindows(
            Some(enum_window_callback),
            &mut hung_pids as *mut _ as isize,
        );
    }
    hung_pids
}
/// Helper: Force terminates a targeted PID directly via Win32 API
pub fn force_kill_pid(pid: u32) -> bool {
    unsafe {
        let h_process = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if h_process == 0 {
            return false;
        }

        let result = TerminateProcess(h_process, 1);
        CloseHandle(h_process);
        result != 0
    }
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

// Spawning a background thread that listens gobally for `Ctrl + Alt + Shift + Esc`

pub fn start_global_hotkey_listener() {
    thread::spawn(|| unsafe {
        let hotkey_id = 1001; //Unique ID for our registered hotkey

        let modifiers = MOD_CONTROL | MOD_ALT | MOD_SHIFT;

        //Register globla hotkey with Windows Kernel

        if RegisterHotKey(0, hotkey_id, modifiers, VK_ESCAPE as u32) == 0 {
            println!("Failed to register global panic Hotkey (Ctrl + Alt + Shift + Esc)");
            return;
        }

        println!("Global Panic Hotkey Registered : [Ctrl + Alt + Shift + Esc]");

        let mut msg: MSG = std::mem::zeroed();

        // Blocking Windows Message Loop listening for WM_HOTKEY
        while GetMessageW(&mut msg, 0, 0, 0) > 0 {
            if msg.message == WM_HOTKEY && msg.wParam == hotkey_id as usize {
                // Instantly terminate whatever app is currently frozen in the foreground!
                emergency_kill_foreground_app();
            }
        }

        //Cleanup on thread exit
        UnregisterHotKey(0, hotkey_id);
    });
}
