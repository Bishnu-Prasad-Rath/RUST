use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ThreatLevel {
    Safe,
    Low(String),
    Medium(String),
    Critical(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ProcessCategory {
    System,
    UserApp,
    BackgroundService,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Normal,
    HighMemoryUsage(u64),
    SecurityRisk(ThreatLevel),
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub memory_mb: u64,
    pub category: ProcessCategory,
    pub status: HealthStatus,
}

impl ProcessInfo {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn impact_score(&self) -> u64 {
        let mut score = self.memory_mb;

        if let HealthStatus::SecurityRisk(ref threat) = self.status {
            match threat {
                ThreatLevel::Critical(_) => score += 10_000,
                ThreatLevel::Medium(_) => score += 5_000,
                ThreatLevel::Low(_) => score += 1_000,
                ThreatLevel::Safe => {}
            }
        }

        match self.category {
            ProcessCategory::UserApp => score += 500,
            ProcessCategory::BackgroundService => score += 100,
            ProcessCategory::System => score += 0,
        }
        score
    }
}