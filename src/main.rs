//! Command-line entry point for LitterPurple.

use anyhow::Result;
use clap::{Parser, Subcommand};
use litterpurple::{service::DeviceService, transport::{MockTransport, SyscfgTransport, UnavailableUsbTransport}};
use log::debug;

/// Safely inspect and update a prepared device's prototype SysCFG fields.
#[derive(Debug, Parser)]
#[command(name = "litterpurple", version, about)]
struct Cli {
    /// Use the in-memory demonstration device instead of USB hardware.
    #[arg(long, global = true, env = "LITTERPURPLE_MOCK")]
    mock: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Detect an externally prepared compatible device.
    Detect,
    /// Display basic device information.
    Info,
    /// Read all fields supported by this prototype.
    Read,
    /// Update one supported field and verify the change.
    Write {
        #[command(subcommand)]
        field: WriteField,
    },
}

#[derive(Debug, Subcommand)]
enum WriteField {
    /// Set the serial number.
    Serial { serial: String },
    /// Set the Wi-Fi MAC address.
    Wifi { mac: String },
    /// Set the Bluetooth MAC address.
    Bt { mac: String },
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    debug!("mock transport: {}", cli.mock);
    if cli.mock {
        run(cli.command, MockTransport::default())
    } else {
        run(cli.command, UnavailableUsbTransport)
    }
}

fn run<T: SyscfgTransport>(command: Command, transport: T) -> Result<()> {
    let mut service = DeviceService::new(transport);
    match command {
        Command::Detect => {
            if service.detect()? { println!("Prepared compatible device detected."); }
        }
        Command::Info | Command::Read => println!("{}", serde_json::to_string_pretty(&service.read_all()?)?),
        Command::Write { field } => {
            let label = match field {
                WriteField::Serial { serial } => {
                    service.write_serial(&serial)?;
                    "Serial number"
                }
                WriteField::Wifi { mac } => {
                    service.write_wifi(&mac)?;
                    "Wi-Fi MAC"
                }
                WriteField::Bt { mac } => {
                    service.write_bluetooth(&mac)?;
                    "Bluetooth MAC"
                }
            };
            println!("{label} updated and verified.");
        }
    }
    Ok(())
}
