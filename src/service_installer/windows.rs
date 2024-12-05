use crate::config::CONFIG;
use log::info;
use std::error::Error;
use std::ffi::OsString;
use std::path::Path;
use windows_service::{
    service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceState, ServiceType},
    service_manager::{ServiceManager, ServiceManagerAccess},
};
use winreg::enums::*;
use winreg::RegKey;

pub fn install() -> Result<(), Box<dyn Error>> {
    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service_binary_path = std::env::current_exe()?;

    let service_info = ServiceInfo {
        name: OsString::from(CONFIG.name),
        display_name: OsString::from(CONFIG.display_name),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::AutoStart,
        error_control: ServiceErrorControl::Normal,
        executable_path: service_binary_path,
        launch_arguments: vec![],
        dependencies: vec![],
        account_name: None,
        account_password: None,
    };

    let service = service_manager.create_service(&service_info, ServiceAccess::CHANGE_CONFIG)?;
    service.set_description(CONFIG.description)?;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("SYSTEM")
        .join("CurrentControlSet")
        .join("Services")
        .join(CONFIG.name);
    let (key, _) = hklm.create_subkey(&path)?;
    key.set_value("Description", &CONFIG.description)?;

    Ok(())
}

pub fn uninstall() -> Result<(), Box<dyn Error>> {
    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE;
    let service = service_manager.open_service(CONFIG.name, service_access)?;

    service.delete()?;
    Ok(())
}

pub fn start() -> Result<(), Box<dyn Error>> {
    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service_access = ServiceAccess::START;
    let service = service_manager.open_service(CONFIG.name, service_access)?;

    service.start(&[])?;
    Ok(())
}

pub fn stop() -> Result<(), Box<dyn Error>> {
    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service_access = ServiceAccess::STOP;
    let service = service_manager.open_service(CONFIG.name, service_access)?;

    service.stop()?;
    Ok(())
}

pub fn status() -> Result<(), Box<dyn Error>> {
    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service_access = ServiceAccess::QUERY_STATUS;
    let service = service_manager.open_service(CONFIG.name, service_access)?;

    let service_status = service.query_status()?;
    match service_status.current_state {
        ServiceState::Running => info!("Service is running"),
        ServiceState::Stopped => info!("Service is stopped"),
        ServiceState::StartPending => info!("Service is starting"),
        ServiceState::StopPending => info!("Service is stopping"),
        ServiceState::ContinuePending => info!("Service is continuing"),
        ServiceState::PausePending => info!("Service is pausing"),
        ServiceState::Paused => info!("Service is paused"),
    }
    Ok(())
}