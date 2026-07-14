//! Transport boundary. USB implementation belongs behind this trait.

use std::collections::HashMap;

use crate::{
    error::{LitterPurpleError, Result},
    model::{DeviceInfo, MacAddress, SyscfgField},
};

/// An implementation that can communicate with an already-prepared device.
pub trait SyscfgTransport {
    /// Reports whether a compatible prepared device is available.
    fn detect(&self) -> Result<bool>;
    /// Reads immutable and editable device data.
    fn device_info(&self) -> Result<DeviceInfo>;
    /// Reads one supported field as text.
    fn read_field(&self, field: SyscfgField) -> Result<String>;
    /// Writes one supported field from a validated text value.
    fn write_field(&mut self, field: SyscfgField, value: &str) -> Result<()>;
}

/// An in-memory device for tests, documentation examples, and UI development.
#[derive(Clone, Debug)]
pub struct MockTransport {
    identifier: String,
    values: HashMap<SyscfgField, String>,
}

impl Default for MockTransport {
    fn default() -> Self {
        let values = HashMap::from([
            (SyscfgField::Serial, "C02LPURPLE01".to_owned()),
            (SyscfgField::Wifi, "02:11:22:33:44:55".to_owned()),
            (SyscfgField::Bluetooth, "02:11:22:33:44:56".to_owned()),
        ]);
        Self { identifier: "Mock A12/A13 device".to_owned(), values }
    }
}

impl SyscfgTransport for MockTransport {
    fn detect(&self) -> Result<bool> {
        Ok(true)
    }

    fn device_info(&self) -> Result<DeviceInfo> {
        Ok(DeviceInfo {
            identifier: self.identifier.clone(),
            serial_number: self.read_field(SyscfgField::Serial)?,
            wifi_mac: self.read_field(SyscfgField::Wifi)?.parse()?,
            bluetooth_mac: self.read_field(SyscfgField::Bluetooth)?.parse()?,
        })
    }

    fn read_field(&self, field: SyscfgField) -> Result<String> {
        self.values.get(&field).cloned().ok_or_else(|| LitterPurpleError::ReadFailure {
            field: field.as_str().to_owned(),
            reason: "field is not present in mock storage".to_owned(),
        })
    }

    fn write_field(&mut self, field: SyscfgField, value: &str) -> Result<()> {
        self.values.insert(field, value.to_owned());
        Ok(())
    }
}

/// Placeholder transport used until the Windows USB implementation is added.
#[derive(Debug, Default)]
pub struct UnavailableUsbTransport;

impl SyscfgTransport for UnavailableUsbTransport {
    fn detect(&self) -> Result<bool> {
        Err(LitterPurpleError::DeviceNotDetected)
    }

    fn device_info(&self) -> Result<DeviceInfo> {
        Err(LitterPurpleError::DeviceNotDetected)
    }

    fn read_field(&self, _field: SyscfgField) -> Result<String> {
        Err(LitterPurpleError::DeviceNotDetected)
    }

    fn write_field(&mut self, _field: SyscfgField, _value: &str) -> Result<()> {
        Err(LitterPurpleError::DeviceNotDetected)
    }
}
