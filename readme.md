# Cross-Platform Rust Service

This project demonstrates a basic cross-platform service implementation in Rust. The service runs as a Windows service on Windows operating systems and as a daemon on Unix-like systems (Linux, macOS).

## Features

- Cross-platform support (Windows, Linux, macOS)
- Basic logging functionality
- Simple service logic (logs a message every minute)

## Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)
- Cargo (usually comes with Rust)

## Building the Project

To build the project, run the following command in the project root directory:

```bash
cargo build --release
```

This will create an optimized executable in the `target/release` directory.

## Running the Service

### On Unix-like systems (Linux, macOS)

1. Navigate to the project directory
2. Run the following command:

```bash
sudo ./target/release/cross_platform_service
```

The service will start as a daemon process. You can check its s`tatus in the log file at `/tmp/service.log`.

### On Windows

1. Open an elevated Command Prompt (Run as Administrator)
2. Navigate to the project directory
3. Install the service:

```bash
sc create RustCrossPlatformService binPath= "full\path\to\target\release\cross_platform_service.exe"
```

4. Start the service:

```bash
sc start RustCrossPlatformService
```

The service will start running as a Windows service. You can check its status in the Windows Event Viewer or in the log file at `C:\Windows\Temp\service.log`.

## Stopping the Service

### On Unix-like systems

To stop the service, you need to find its process ID and kill it:

1. Find the PID:
   ```
   cat /tmp/service.pid
   ```
2. Kill the process:
   ```
   sudo kill -15 <PID>
   ```
### On Windows

To stop the Windows service:

## Uninstalling the Service

### On Unix-like systems

Simply stop the service as described above. You may also want to remove the log and PID files:

```bash
sudo rm /tmp/service.log /tmp/service.pid
```

### On Windows

1. Stop the service if it's running:
   ```
   sc stop RustCrossPlatformService
   ```
2. Delete the service:
   ```
   sc delete RustCrossPlatformService
   ```

## Customizing the Service

To modify the service behavior, edit the `run_service()` function in `src/service_logic.rs`. You can add your own logic there to perform whatever tasks you need your service to do.

## License

This project is open source and available under the [Apache License 2.0](LICENSE).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.