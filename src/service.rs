//! Application operations and write-readback verification.

use crate::{
    error::{LitterPurpleError, Result},
    model::{DeviceInfo, SyscfgField},
    transport::SyscfgTransport,
    validation::{validate_mac, validate_serial},
};

/// High-level use cases shared by the CLI and a future desktop application.
pub struct DeviceService<T> {
    transport: T,
}

impl<T: SyscfgTransport> DeviceService<T> {
    /// Creates a service around one transport instance.
    #[must_use]
    pub const fn new(transport: T) -> Self {
        Self { transport }
    }

    /// Detects a prepared and compatible device.
    pub fn detect(&self) -> Result<bool> {
        self.transport.detect()
    }

    /// Retrieves basic device information.
    pub fn info(&self) -> Result<DeviceInfo> {
        self.transport.device_info()
    }

    /// Retrieves every field exposed in the initial prototype.
    pub fn read_all(&self) -> Result<DeviceInfo> {
        self.info()
    }

    /// Writes a serial number and verifies the readback value.
    pub fn write_serial(&mut self, value: &str) -> Result<()> {
        validate_serial(value)?;
        self.write_verified(SyscfgField::Serial, value)
    }

    /// Writes a Wi-Fi MAC address and verifies the normalized readback value.
    pub fn write_wifi(&mut self, value: &str) -> Result<()> {
        validate_mac(value)?;
        self.write_verified(SyscfgField::Wifi, value)
    }

    /// Writes a Bluetooth MAC address and verifies the normalized readback value.
    pub fn write_bluetooth(&mut self, value: &str) -> Result<()> {
        validate_mac(value)?;
        self.write_verified(SyscfgField::Bluetooth, value)
    }

    fn write_verified(&mut self, field: SyscfgField, value: &str) -> Result<()> {
        self.transport.write_field(field, value)?;
        let actual = self.transport.read_field(field)?;
        let matches = match field {
            SyscfgField::Serial => actual == value,
            SyscfgField::Wifi | SyscfgField::Bluetooth => actual.eq_ignore_ascii_case(value),
        };
        if matches {
            Ok(())
        } else {
            Err(LitterPurpleError::VerificationFailure {
                field: field.as_str().to_owned(),
                expected: value.to_owned(),
                actual,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{model::SyscfgRecord, transport::MockTransport};
    use super::*;

    #[test]
    fn verifies_mock_serial_write() {
        let mut service = DeviceService::new(MockTransport::default());
        assert!(service.write_serial("C02-NEW123").is_ok());
        assert_eq!(service.info().unwrap().serial_number, "C02-NEW123");
    }

    #[test]
    fn parses_length_prefixed_record() {
        assert_eq!(SyscfgRecord::parse(&[3, 1, 2, 3]).unwrap().payload, vec![1, 2, 3]);
    }

    #[test]
    fn rejects_wrong_record_length() {
        assert!(SyscfgRecord::parse(&[3, 1, 2]).is_err());
    }
}
