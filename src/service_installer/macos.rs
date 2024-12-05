use crate::config::{self, CONFIG};
use log::info;
use std::error::Error;
use std::fs;
use std::process::Command;

pub fn install() -> Result<(), Box<dyn Error>> {
    let paths = config::get_service_paths();
    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>{}</string>
    <key>StandardErrorPath</key>
    <string>{}</string>
</dict>
</plist>"#,
        CONFIG.name,
        std::env::current_exe()?.to_str().unwrap(),
        CONFIG.log_path.display(),
        CONFIG.error_log_path.display()
    );

    fs::write(&paths.service_file, plist_content)?;
    Command::new("launchctl")
        .args(&["load", paths.service_file.to_str().unwrap()])
        .status()?;

    Ok(())
}

pub fn uninstall() -> Result<(), Box<dyn Error>> {
    let paths = config::get_service_paths();
    Command::new("launchctl")
        .args(&["unload", paths.service_file.to_str().unwrap()])
        .status()?;
    fs::remove_file(&paths.service_file)?;
    Ok(())
}

pub fn start() -> Result<(), Box<dyn Error>> {
    Command::new("launchctl")
        .args(&["start", CONFIG.name])
        .status()?;
    Ok(())
}

pub fn stop() -> Result<(), Box<dyn Error>> {
    Command::new("launchctl")
        .args(&["stop", CONFIG.name])
        .status()?;
    Ok(())
}

pub fn status() -> Result<(), Box<dyn Error>> {
    let output = Command::new("launchctl")
        .args(&["list", CONFIG.name])
        .output()?;
    
    if output.status.success() {
        info!("Service is running");
    } else {
        info!("Service is not running");
    }
    Ok(())
}