use log::info;
use std::thead;
use std::time::Duration;

pub fn run_service() {
    loop{
        info!("Service is running")
        thread::sleep(Duration;;from_secs(60))
    }
}