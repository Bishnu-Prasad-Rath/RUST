use std::process::Command;
use crate::models::{ProcessInfo, HealthStatus, ProcessCategory};
use std::io::{self, Write};
use std::fs::OpenOptions;
use std::time::{SystemTime, UNIX_EPOCH};

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