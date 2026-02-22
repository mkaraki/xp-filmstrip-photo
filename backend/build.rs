use chrono::Local;

fn main() {
    // Format: YYYY.MMDD.HHMMSS
    let build_time = Local::now().format("%Y.%m%d.%H%M%S").to_string();
    
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
}
