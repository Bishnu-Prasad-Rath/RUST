use std::process::Command;
use crate::models::{ProcessInfo, HealthStatus, ProcessCategory};
use std::io::{self, Write};
use std::fs::OpenOptions;
use std::time::{SystemTime, UNIX_EPOCH};
use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
  CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::Threading::{
  OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
};
use windows_sys::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};

pub fn log_event(event_type: &str, details: &str){
  let start = SystemTime::now();
  let timestamp = start.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
  let log_entry = format!("[TIMESTAMP: {}] [{}] {}\n", timestamp, event_type, details);

  if let Ok(mut file) = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("sentinel.log"){
    let _ = file.write_all(log_entry.as_bytes());
                        }
}

pub fn fetch_live_processes_win32()-> Result<Vec<ProcessInfo>, String>{
  let mut processes = Vec::new();

  unsafe{
    // 1.Take an atomic snapshot of all running processes of the system
    let snapshot_handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
      if snapshot_handle == INVALID_HANDLE_VALUE{
        return Err("Failed to create Win32 toolhelp process snapshot.".to_string());
      }

  // 2. Initialize the Windows PROCESSENTRY32W C-struct
  let mut entry: PROCESSENTRY32W = std::mem::zeroed();
  entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

  // 3. Retrieve the first process from the sapshot
  if Process32FirstW(snapshot_handle, &mut entry) != 0{
    loop{
      let pid = entry.th32ProcessID;

      // Convert the null terminated UTF-16 wide string (entry.szExeFile) to standard RUST string
        let name_len = entry.szExeFile.iter().position(|&c| c==0).unwrap_or(entry.szExeFile.len());
        let name = String::from_utf16_lossy(&entry.szExeFile[..name_len]);

        if pid > 0 && !name.is_empty(){
          // Categorize based on process name
          let name_lower = name.to_lowercase();
          let category = if name_lower.contains("svc") || name_lower.contains("system"){
            ProcessCategory::System
          }else if name_lower.contains("service"){
            ProcessCategory::BackgroundService
          }else{
            ProcessCategory::UserApp
          };

          // For raw Win32 process enumeration, memory sis fetched via handles.
          // I assign a default placeholder here or calculate exact working sets in the next phase

          let memory_mb = get_process_memory(pid);
          
        processes.push(ProcessInfo {
                        pid,
                        name,
                        memory_mb,
                        category,
                        status: HealthStatus::Normal,
                    });
        }
          //Move to the next process in the snapshot loop, break when return 0.
          if Process32NextW(snapshot_handle, &mut entry) == 0{
            break;
          }
    }
  }
    CloseHandle(snapshot_handle);    //For preventing kernel resource leak
  }
    Ok(processes)
}

  // This function is for querying OpenProcess and GetProcessMemoryInfo

fn get_process_memory(pid: u32) -> u64 {
    unsafe {
        // Request query and VM read rights for process handle
        let h_process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        
        // Fix: In windows-sys, HANDLE is an isize, so check against 0 or INVALID_HANDLE_VALUE
        if h_process == 0 || h_process == INVALID_HANDLE_VALUE {
            return 0; // Protected system processes (like csrss.exe) won't grant handles without Admin elevation
        }

        let mut pmc: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
        pmc.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;

        let memory_mb = if GetProcessMemoryInfo(
            h_process,
            &mut pmc,
            std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        ) != 0
        {
            // WorkingSetSize is in bytes -> Convert to Megabytes (MB)
            (pmc.WorkingSetSize / (1024 * 1024)) as u64
        } else {
            0
        };

        CloseHandle(h_process);
        memory_mb
    }
}



pub fn fetch_live_processes()->Result<Vec<ProcessInfo>,String>{
  let output = Command::new("cmd")
               .args(&["/C","tasklist"])
               .output()
               .map_err(|e| format!("Failed to execute command {}",e))?;

  let raw_text = String::from_utf8_lossy(&output.stdout);
  let mut processes = Vec::new();

  for line in raw_text.lines().skip(3){
    let parts : Vec<&str> = line.split_whitespace().collect();

  if parts.len() >= 5 {
    let name = parts[0].to_string();

    if let Ok(pid) = parts[1].parse::<u32>(){
      let mem_str = parts[parts.len() - 2].replace(",","");
      let memory_kb = mem_str.parse::<u64>().unwrap_or(0);
      let memory_mb = memory_kb / 1024;  //For KB->MB

      let name_lower = name.to_lowercase();
      let category = if name_lower.contains("svc") || name_lower.contains("system"){
        ProcessCategory::System
      }else if name_lower.contains("serice"){
      ProcessCategory::BackgroundService
      }else{
       ProcessCategory::UserApp
      };
  processes.push(ProcessInfo::new(pid, name, memory_mb, category));
    }
  }
  }
  Ok(processes)
}

pub fn ask_user_confirmation(prompt_message:&str)->bool{
  print!("{} (Y/N) : ",prompt_message);

  io::stdout().flush().unwrap_or(());

  let mut input = String::new();

  if io::stdin().read_line(&mut input).is_ok(){
    let cleaned = input.trim();
    return cleaned.eq_ignore_ascii_case("y") || cleaned.eq_ignore_ascii_case("yes");
  }
  false
}

pub fn kill_process(proc: &mut ProcessInfo) -> bool {
    println!("~Attempting to terminate PID {} ({}) via Windows TaskKill...", proc.pid, proc.name);

    let output = Command::new("taskkill")
        .args(&["/F", "/PID", &proc.pid.to_string()])
        .output();

    match output {
        Ok(res) if res.status.success() => {
            proc.status = HealthStatus::Terminated;
            println!("~Successfully terminated process {}!", proc.name);

            log_event("TERMINATED",&format!("Successfully killed {} (PID: {})",proc.name,proc.pid));
            true
        }
        Ok(_res) => {
            println!("~Failed to terminate process {}!", proc.name);
            false
        }
        Err(e) => {
            println!("~Error while executing taskkill command: {}", e);
            false
        }
    }
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}