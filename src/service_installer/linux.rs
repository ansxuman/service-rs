use crate::config::{self, CONFIG};
use log::info;
use std::error::Error;
use std::fs;
use std::process::Command;

pub fn install() -> Result<(), Box<dyn Error>> {
    let paths = config::get_service_paths();
    let service_content = format!(
        r#"[Unit]
Description={}
After=network.target

[Service]
ExecStart={}
Restart=always
User=root
StandardOutput=append:{}
StandardError=append:{}

[Install]
WantedBy=multi-user.target"#,
        CONFIG.description,
        std::env::current_exe()?.to_str().unwrap(),
        CONFIG.log_path.display(),
        CONFIG.error_log_path.display()
    );

    fs::write(&paths.service_file, service_content)?;
    Command::new("systemctl").args(&["daemon-reload"]).status()?;
    Command::new("systemctl").args(&["enable", CONFIG.name]).status()?;

    Ok(())
}

pub fn uninstall() -> Result<(), Box<dyn Error>> {
    let paths = config::get_service_paths();
    Command::new("systemctl").args(&["stop", CONFIG.name]).status()?;
    Command::new("systemctl").args(&["disable", CONFIG.name]).status()?;
    fs::remove_file(&paths.service_file)?;
    Command::new("systemctl").args(&["daemon-reload"]).status()?;
    Ok(())
}

pub fn start() -> Result<(), Box<dyn Error>> {
    Command::new("systemctl").args(&["start", CONFIG.name]).status()?;
    Ok(())
}

pub fn stop() -> Result<(), Box<dyn Error>> {
    Command::new("systemctl").args(&["stop", CONFIG.name]).status()?;
    Ok(())
}

pub fn status() -> Result<(), Box<dyn Error>> {
    let output = Command::new("systemctl")
        .args(&["is-active", CONFIG.name])
        .output()?;
    
    if output.status.success() {
        info!("Service is running");
    } else {
        info!("Service is not running");
    }
    Ok(())
}