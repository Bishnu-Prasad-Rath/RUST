#[derive(Debug, PartialEq, Clone)]
pub enum ThreatLevel{
  Safe,
  Low(String),
  Medium(String),
  Critical(String),
}


#[derive(Debug, PartialEq, Clone)]
pub enum ProcessCategory {
    System,
    UserApp,
    BackgroundService,
}

#[derive(Debug, PartialEq, Clone)]
pub enum HealthStatus {
    Normal,
    HighMemoryUsage(u64),
    SecurityRisk(ThreatLevel),
    Terminated,
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub memory_mb: u64,
    pub category: ProcessCategory,
    pub status: HealthStatus,
}

impl ProcessInfo {
    pub fn new(pid: u32, name: String, memory_mb: u64, category: ProcessCategory) -> Self {
        let status = if memory_mb > 500 {
            HealthStatus::HighMemoryUsage(memory_mb)
        } else {
            HealthStatus::Normal
        };

        ProcessInfo {
            pid,
            name,
            memory_mb,
            category,
            status,
        }
    }
pub fn impact_score(&self)->u64{
  let mut score = self.memory_mb;

  if let HealthStatus::SecurityRisk(ref threat) = self.status{
    match threat{
      ThreatLevel::Critical(_)=> score += 10_000,
      ThreatLevel::Medium(_)=> score += 5_000,
      ThreatLevel::Low(_)=> score += 1_000,
      ThreatLevel::Safe=> {},
    }
  }

match self.category{
  ProcessCategory::UserApp => score += 500,
  ProcessCategory::BackgroundService => score += 100,
  ProcessCategory::System => score += 0,
}
  score
}

pub fn print_status(&self) {
        let cat_label = match self.category {
            ProcessCategory::System => "SYSTEM",
            ProcessCategory::UserApp => "USER APP",
            ProcessCategory::BackgroundService => "SERVICE",
        };

        print!("[PID {:>5}] {:<22} | Type: {:<10}", self.pid, self.name, cat_label);

        match &self.status {
            HealthStatus::SecurityRisk(threat) => match threat {
                ThreatLevel::Critical(msg) => println!(" | 🛑 CRITICAL THREAT: {}", msg),
                ThreatLevel::Medium(msg)   => println!(" | ⚠️ WARNING: {}", msg),
                ThreatLevel::Low(msg)      => println!(" | ℹ️ NOTICE: {}", msg),
                ThreatLevel::Safe          => println!(" | ✅ OK"),
            },
            HealthStatus::HighMemoryUsage(mem) => {
                println!(" | ⚠️ HIGH MEMORY ({} MB)", mem);
            }
            HealthStatus::Normal => {
                println!(" | ✅ OK");
            }
            HealthStatus::Terminated => {
                println!(" | ❌ TERMINATED");
            }
        }
    }
}