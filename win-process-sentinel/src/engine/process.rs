use crate::models::{HealthStatus, ProcessCategory, ProcessInfo};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
use windows_sys::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, TerminateProcess, PROCESS_QUERY_INFORMATION,
    PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_TERMINATE, PROCESS_VM_READ,
};

/// Hardware metrics captured from host OS
#[derive(Debug, Clone, Copy)]
pub struct SystemMemoryMetrics {
    pub total_ram_mb: u64,
    pub available_ram_mb: u64,
    pub memory_load_pct: u32,
    pub dynamic_app_threshold_mb: u64,
}

/// Dynamically inspects host hardware RAM using Win32 GlobalMemoryStatusEx
pub fn get_system_memory_metrics() -> SystemMemoryMetrics {
    unsafe {
        let mut mem_status: MEMORYSTATUSEX = std::mem::zeroed();
        mem_status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

        if GlobalMemoryStatusEx(&mut mem_status) != 0 {
            let total_ram_mb = mem_status.ullTotalPhys / (1024 * 1024);
            let available_ram_mb = mem_status.ullAvailPhys / (1024 * 1024);
            let memory_load_pct = mem_status.dwMemoryLoad;

            // Dynamically scale limit to 15% of total system RAM (minimum 600 MB floor)
            let dynamic_app_threshold_mb = ((total_ram_mb as f64 * 0.15) as u64).max(600);

            SystemMemoryMetrics {
                total_ram_mb,
                available_ram_mb,
                memory_load_pct,
                dynamic_app_threshold_mb,
            }
        } else {
            // Fallback safe defaults if kernel query fails
            SystemMemoryMetrics {
                total_ram_mb: 8192,
                available_ram_mb: 4096,
                memory_load_pct: 50,
                dynamic_app_threshold_mb: 1200,
            }
        }
    }
}

pub fn log_event(event_type: &str, details: &str) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let log_entry = format!("[TIMESTAMP: {}] [{}] {}\n", timestamp, event_type, details);

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("sentinel.log")
    {
        let _ = file.write_all(log_entry.as_bytes());
    }
}

pub fn fetch_live_processes_win32() -> Result<Vec<ProcessInfo>, String> {
    let mut processes = Vec::new();

    unsafe {
        let snapshot_handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot_handle == INVALID_HANDLE_VALUE || snapshot_handle == 0 {
            return Err("Failed to create Win32 toolhelp process snapshot.".to_string());
        }

        let mut entry: PROCESSENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        if Process32FirstW(snapshot_handle, &mut entry) != 0 {
            loop {
                let pid = entry.th32ProcessID;

                let name_len = entry
                    .szExeFile
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(entry.szExeFile.len());
                let name = String::from_utf16_lossy(&entry.szExeFile[..name_len]);

                if pid > 0 && !name.is_empty() {
                    let name_lower = name.to_lowercase();
                    let category = if name_lower.contains("svc") || name_lower.contains("system") {
                        ProcessCategory::System
                    } else if name_lower.contains("service") {
                        ProcessCategory::BackgroundService
                    } else {
                        ProcessCategory::UserApp
                    };

                    let memory_mb = get_process_memory(pid);

                    processes.push(ProcessInfo {
                        pid,
                        name,
                        memory_mb,
                        category,
                        status: HealthStatus::Normal,
                    });
                }

                if Process32NextW(snapshot_handle, &mut entry) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snapshot_handle);
    }
    Ok(processes)
}

pub fn get_process_memory(pid: u32) -> u64 {
    unsafe {
        let h_process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        if h_process == 0 || h_process == INVALID_HANDLE_VALUE {
            return 0;
        }

        let mut pmc: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
        pmc.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;

        let memory_mb = if GetProcessMemoryInfo(
            h_process,
            &mut pmc,
            std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        ) != 0
        {
            (pmc.WorkingSetSize / (1024 * 1024)) as u64
        } else {
            0
        };

        CloseHandle(h_process);
        memory_mb
    }
}

pub fn get_process_exe_path(pid: u32) -> Option<String> {
    unsafe {
        let h_process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if h_process == 0 || h_process == INVALID_HANDLE_VALUE {
            return None;
        }

        let mut buffer: [u16; 1024] = [0; 1024];
        let mut size = buffer.len() as u32;

        let success = QueryFullProcessImageNameW(h_process, 0, buffer.as_mut_ptr(), &mut size);
        CloseHandle(h_process);

        if success != 0 {
            Some(String::from_utf16_lossy(&buffer[..size as usize]))
        } else {
            None
        }
    }
}

pub fn force_kill_pid(pid: u32) -> bool {
    unsafe {
        let h_process = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if h_process == 0 || h_process == INVALID_HANDLE_VALUE {
            return false;
        }

        let result = TerminateProcess(h_process, 1);
        CloseHandle(h_process);
        result != 0
    }
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
