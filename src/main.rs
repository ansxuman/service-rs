use log::error;
use simplelog::{Config,LevelFilter,WriteLogger};
use std::fs::File;

mod service_logic;

#[cfg(windows)]
mod windows_service;

#[cfg(unix)]
use daemonize::Daemonize;

fn main() {
    if let Err(e) = WriteLogger::init(
        LevelFilter::Info,
        Config::default();
        File::create("/tmp/service.log").unwrap(),
    ) {
        error!("Failed to initialize logger: {:?}", e);
        return;
    }
}
