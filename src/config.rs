use std::path::PathBuf;

pub struct ServiceConfig {
    pub name: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
    pub log_path: PathBuf,
    pub error_log_path: PathBuf,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            name: "com.service.rs",
            display_name: "Cross Platform Rust Service",
            description: "A cross-platform service written in Rust",
            log_path: PathBuf::from("/tmp/service_rs.log"),
            error_log_path: PathBuf::from("/tmp/service_rs.err"),
        }
    }
}

#[cfg(target_os = "macos")]
pub fn get_service_paths() -> ServicePaths {
    ServicePaths {
        service_file: PathBuf::from("/Library/LaunchDaemons/com.service.rs.plist"),
        log_dir: PathBuf::from("/tmp"),
    }
}

#[cfg(target_os = "linux")]
pub fn get_service_paths() -> ServicePaths {
    ServicePaths {
        service_file: PathBuf::from("/etc/systemd/system/com.service.rs.service"),
        log_dir: PathBuf::from("/var/log"),
    }
}

pub struct ServicePaths {
    pub service_file: PathBuf,
    pub log_dir: PathBuf,
}

lazy_static::lazy_static! {
    pub static ref CONFIG: ServiceConfig = ServiceConfig::default();
}