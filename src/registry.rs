use std::env;
use winreg::enums::*;
use winreg::RegKey;

pub fn register_browser() -> Result<(), Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = env::current_exe()?;
    let path_str = path.to_str().unwrap();

    // Register as a browser in Windows
    let (app_key, _) = hkcu.create_subkey(r"Software\Classes\Browsea")?;
    app_key.set_value("", &"Browsea")?;

    register_capabilities(&app_key, path_str)?;
    register_client(&hkcu, path_str)?;
    register_default_programs(&hkcu)?;

    println!("Browser registered successfully!");
    Ok(())
}

fn register_capabilities(app_key: &RegKey, path_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (cap_key, _) = app_key.create_subkey(r"Capabilities")?;
    cap_key.set_value("ApplicationName", &"Browsea")?;
    cap_key.set_value("ApplicationDescription", &"Choose which browser to open links with")?;

    let (url_key, _) = cap_key.create_subkey("URLAssociations")?;
    url_key.set_value("http", &"Browsea")?;
    url_key.set_value("https", &"Browsea")?;

    let (cmd_key, _) = app_key.create_subkey(r"shell\open\command")?;
    cmd_key.set_value("", &format!("\"{}\" \"%1\"", path_str))?;

    Ok(())
}

fn register_client(hkcu: &RegKey, path_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (client_key, _) = hkcu.create_subkey(r"Software\Clients\StartMenuInternet\Browsea")?;
    client_key.set_value("", &"Browsea")?;

    let (client_cap_key, _) = client_key.create_subkey("Capabilities")?;
    client_cap_key.set_value("ApplicationName", &"Browsea")?;
    client_cap_key.set_value("ApplicationDescription", &"Choose which browser to open links with")?;

    let (client_cmd_key, _) = client_key.create_subkey(r"shell\open\command")?;
    client_cmd_key.set_value("", &format!("\"{}\" \"%1\"", path_str))?;

    Ok(())
}

fn register_default_programs(hkcu: &RegKey) -> Result<(), Box<dyn std::error::Error>> {
    let (reg_apps, _) = hkcu.create_subkey(r"Software\RegisteredApplications")?;
    reg_apps.set_value("Browsea", &r"Software\Classes\Browsea\Capabilities")?;
    Ok(())
} 