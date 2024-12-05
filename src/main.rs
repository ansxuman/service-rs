use clap::{Parser, Subcommand};
use log::error;
use simplelog::{Config, LevelFilter, WriteLogger};

mod config;
mod service_logic;
mod service_installer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Install,
    Uninstall,
    Start,
    Stop,
    Restart,
    Status,
}

fn main() {
    if let Err(e) = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        std::fs::File::create(&config::CONFIG.log_path).unwrap(),
    ) {
        eprintln!("Failed to initialize logger: {:?}", e);
        return;
    }

    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Install) => service_installer::install(),
        Some(Commands::Uninstall) => service_installer::uninstall(),
        Some(Commands::Start) => service_installer::start(),
        Some(Commands::Stop) => service_installer::stop(),
        Some(Commands::Restart) => {
            service_installer::stop().and_then(|_| service_installer::start())
        }
        Some(Commands::Status) => service_installer::status(),
        None => Ok(service_logic::run_service()),
    };

    if let Err(e) = result {
        error!("Operation failed: {:?}", e);
    }
}