use std::process::Command;

pub fn inspect_installed_apps_and_services() {
    println!("==========================================================================");
    println!("   🔍 SENTINEL SYSTEM INVENTORY & INSTALLED APPLICATIONS");
    println!("==========================================================================");

    println!("\n📦 Installed Applications & Packages:");
    let _ = Command::new("powershell")
        .args([
            "-Command",
            "Get-ItemProperty HKLM:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\* | \
             Select-Object DisplayName, DisplayVersion, InstallDate, EstimatedSize | \
             Where-Object DisplayName -ne $null | Format-Table -AutoSize",
        ])
        .status();

    println!("\n⚙️ Active Windows Services:");
    let _ = Command::new("powershell")
        .args([
            "-Command",
            "Get-Service | Select-Object Status, Name, DisplayName | \
             Where-Object Status -eq 'Running' | Select-Object -First 15 | Format-Table -AutoSize",
        ])
        .status();
}
