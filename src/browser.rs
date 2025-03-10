use std::{env, fs};
use winreg::enums::*;
use winreg::RegKey;

pub fn get_installed_browsers() -> Vec<(String, String)> {
    let mut browsers = Vec::new();
    
    // Check registry for installed browsers
    let registry_paths = vec![
        // Standard paths
        ("Chrome", r"Software\Clients\StartMenuInternet\Google Chrome\shell\open\command"),
        ("Chrome", r"Software\Microsoft\Windows\CurrentVersion\App Paths\chrome.exe"),
        ("Firefox", r"Software\Clients\StartMenuInternet\FIREFOX.EXE\shell\open\command"),
        ("Firefox", r"Software\Mozilla\Mozilla Firefox\CommandLineArgs"),
        ("Firefox", r"Software\Microsoft\Windows\CurrentVersion\App Paths\firefox.exe"),
        ("Edge", r"Software\Clients\StartMenuInternet\Microsoft Edge\shell\open\command"),
        ("Edge", r"Software\Microsoft\Windows\CurrentVersion\App Paths\msedge.exe"),
        ("Brave", r"Software\Clients\StartMenuInternet\Brave\shell\open\command"),
        ("Brave", r"Software\Microsoft\Windows\CurrentVersion\App Paths\brave.exe"),
        ("Opera", r"Software\Clients\StartMenuInternet\Opera\shell\open\command"),
        ("Opera", r"Software\Microsoft\Windows\CurrentVersion\App Paths\opera.exe"),
        ("Opera GX", r"Software\Clients\StartMenuInternet\Opera GX\shell\open\command"),
        ("Vivaldi", r"Software\Clients\StartMenuInternet\Vivaldi\shell\open\command"),
        ("DuckDuckGo", r"Software\Clients\StartMenuInternet\DuckDuckGo\shell\open\command"),
        ("DuckDuckGo", r"Software\Microsoft\Windows\CurrentVersion\App Paths\duckduckgo.exe"),
    ];

    check_registry_browsers(&mut browsers, &registry_paths);

    // Check common installation paths
    let program_files = vec![
        env::var("ProgramFiles").unwrap_or_default(),
        env::var("ProgramFiles(x86)").unwrap_or_default(),
        env::var("LocalAppData").unwrap_or_default(),
    ];

    let browser_paths = vec![
        ("Chrome", r"Google\Chrome\Application\chrome.exe"),
        ("Chrome Beta", r"Google\Chrome Beta\Application\chrome.exe"),
        ("Chrome Canary", r"Google\Chrome SxS\Application\chrome.exe"),
        ("Firefox", r"Mozilla Firefox\firefox.exe"),
        ("Firefox", r"Firefox\firefox.exe"),
        ("Firefox Beta", r"Mozilla Firefox Beta\firefox.exe"),
        ("Firefox Developer", r"Firefox Developer Edition\firefox.exe"),
        ("Firefox Nightly", r"Firefox Nightly\firefox.exe"),
        ("Edge", r"Microsoft\Edge\Application\msedge.exe"),
        ("Edge Beta", r"Microsoft\Edge Beta\Application\msedge.exe"),
        ("Edge Dev", r"Microsoft\Edge Dev\Application\msedge.exe"),
        ("Edge Canary", r"Microsoft\Edge SxS\Application\msedge.exe"),
        ("Brave", r"BraveSoftware\Brave-Browser\Application\brave.exe"),
        ("Brave Beta", r"BraveSoftware\Brave-Browser-Beta\Application\brave.exe"),
        ("Opera", r"Opera\launcher.exe"),
        ("Opera", r"Opera\opera.exe"),
        ("Opera GX", r"Opera Software\Opera GX\launcher.exe"),
        ("Opera GX", r"Opera Software\Opera GX\opera.exe"),
        ("Vivaldi", r"Vivaldi\Application\vivaldi.exe"),
        ("Tor Browser", r"Tor Browser\Browser\firefox.exe"),
        ("Waterfox", r"Waterfox\waterfox.exe"),
        ("Pale Moon", r"Pale Moon\palemoon.exe"),
        ("DuckDuckGo", r"DuckDuckGo\DuckDuckGo.exe"),
        ("DuckDuckGo", r"DuckDuckGo\Browser\DuckDuckGo.exe"),
    ];

    check_filesystem_browsers(&mut browsers, &program_files, &browser_paths);

    // Remove duplicates, keeping the first occurrence
    browsers.dedup_by(|(name1, path1), (name2, path2)| {
        name1 == name2 || path1 == path2
    });

    browsers
}

fn check_registry_browsers(browsers: &mut Vec<(String, String)>, registry_paths: &[(&str, &str)]) {
    let registry_roots = vec![HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER];

    for (browser_name, reg_path) in registry_paths {
        for root in &registry_roots {
            if let Ok(key) = RegKey::predef(*root).open_subkey(reg_path) {
                if let Ok(browser_cmd) = key.get_value::<String, _>("") {
                    if let Some(browser_path) = browser_cmd.split('"').nth(1) {
                        if fs::metadata(browser_path).is_ok() {
                            browsers.push((browser_name.to_string(), browser_path.to_string()));
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn check_filesystem_browsers(browsers: &mut Vec<(String, String)>, program_files: &[String], browser_paths: &[(&str, &str)]) {
    for (name, rel_path) in browser_paths {
        if !browsers.iter().any(|(n, _)| n == name) {
            for base in program_files {
                let full_path = format!("{}/{}", base, rel_path).replace("/", "\\");
                if fs::metadata(&full_path).is_ok() {
                    browsers.push((name.to_string(), full_path));
                    break;
                }
            }
        }
    }
}

pub fn extract_domain(url: &str) -> Option<String> {
    if let Some(url) = url.strip_prefix("http://") {
        return url.split('/').next().map(|s| s.to_string());
    } else if let Some(url) = url.strip_prefix("https://") {
        return url.split('/').next().map(|s| s.to_string());
    }
    None
} 