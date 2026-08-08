use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, ContentArrangement, Table};
use crate::models::{ProcessInfo, ProcessCategory, HealthStatus, ThreatLevel};

pub fn render_dashboard(processes: &[ProcessInfo], max_ram: u64) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    // Set 5 Headers
    table.set_header(vec![
        Cell::new("PID").fg(Color::Cyan),
        Cell::new("Process Name").fg(Color::Cyan),
        Cell::new("Category").fg(Color::Cyan),
        Cell::new("Memory").fg(Color::Cyan),
        Cell::new("Health & Security Status").fg(Color::Cyan),
    ]);

    for proc in processes {
        // 1. Category Cell & Color
        let (cat_text, cat_color) = match proc.category {
            ProcessCategory::UserApp => ("User App", Color::Blue),
            ProcessCategory::BackgroundService => ("Service", Color::Magenta),
            ProcessCategory::System => ("System Core", Color::Cyan),
        };

        // 2. Memory Cell & Color
        let mem_text = format!("{} MB", proc.memory_mb);
        let mem_color = if proc.memory_mb >= max_ram {
            Color::Yellow
        } else {
            Color::White
        };

        // 3. Health & Security Status Cell & Color
        let (status_text, status_color) = match &proc.status {
            HealthStatus::SecurityRisk(threat) => match threat {
                ThreatLevel::Critical(msg) => (format!("🛑 THREAT: {}", msg), Color::Red),
                ThreatLevel::Medium(msg)   => (format!("⚠️ WARN: {}", msg), Color::Yellow),
                ThreatLevel::Low(msg)      => (format!("ℹ️ NOTICE: {}", msg), Color::Blue),
                ThreatLevel::Safe          => ("✅ OK".to_string(), Color::Green),
            },
            HealthStatus::HighMemoryUsage(mem) => (format!("⚠️ HIGH RAM ({} MB)", mem), Color::Yellow),
            HealthStatus::Normal => ("✅ OK".to_string(), Color::Green),
            HealthStatus::Terminated => ("❌ TERMINATED".to_string(), Color::DarkRed),
        };

        // Add row using native comfy-table cell coloring
        table.add_row(vec![
            Cell::new(proc.pid),
            Cell::new(&proc.name),
            Cell::new(cat_text).fg(cat_color),
            Cell::new(mem_text).fg(mem_color),
            Cell::new(status_text).fg(status_color),
        ]);
    }

    println!("{}", table);
}