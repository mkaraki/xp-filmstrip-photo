use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Format: YYYY.MMDD.HHMMSS
    // We'll use a simple approach to avoid heavy dependencies in build.rs
    // For precision as requested by user (YYYY.MMDD.HHMMSS), we'll just use the raw timestamp 
    // or try to format it if possible. 
    // Actually, let's just pass the timestamp and let the backend/frontend format it, 
    // or use a shell command to get formatted string.
    
    let output = std::process::Command::new("powershell")
        .args(&["-NoProfile", "-Command", "Get-Date -Format 'yyyy.MMdd.HHmmss'"])
        .output()
        .expect("failed to execute process");
    
    let build_time = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
}
