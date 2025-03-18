use std::process::Command;

/// Launches a browser with the specified URL
pub fn launch_browser(browser_path: &str, url: &str) -> Result<(), String> {
    match Command::new(browser_path)
        .arg(url)
        .spawn() {
            Ok(_) => {
                println!("Successfully launched browser: {}", browser_path);
                Ok(())
            },
            Err(e) => {
                let error_msg = format!("Failed to launch browser: {} - Error: {}", browser_path, e);
                eprintln!("{}", error_msg);
                Err(error_msg)
            }
        }
}
