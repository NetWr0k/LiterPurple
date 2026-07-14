//! Domain models independent of command-line or USB implementation details.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{error::LitterPurpleError, validation::validate_mac};

/// A six-octet media access control address.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MacAddress([u8; 6]);

impl MacAddress {
    /// Creates an address from its six raw octets.
    #[must_use]
    pub const fn new(octets: [u8; 6]) -> Self {
        Self(octets)
    }

    /// Returns the raw octets in network order.
    #[must_use]
    pub const fn octets(self) -> [u8; 6] {
        self.0
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d, e, f] = self.0;
        write!(formatter, "{a:02X}:{b:02X}:{c:02X}:{d:02X}:{e:02X}:{f:02X}")
    }
}

impl FromStr for MacAddress {
    type Err = LitterPurpleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        validate_mac(value)?;
        let mut octets = [0_u8; 6];
        for (index, pair) in value.split(':').enumerate() {
            octets[index] = u8::from_str_radix(pair, 16)
                .map_err(|_| LitterPurpleError::InvalidMac(value.to_owned()))?;
        }
        Ok(Self(octets))
    }
}

impl Serialize for MacAddress {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for MacAddress {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

/// The fields supported by the initial prototype.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyscfgField {
    /// Device serial number.
    Serial,
    /// Wi-Fi MAC address.
    Wifi,
    /// Bluetooth MAC address.
    Bluetooth,
}

impl SyscfgField {
    /// Returns a CLI-friendly field name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Serial => "serial number",
            Self::Wifi => "Wi-Fi MAC",
            Self::Bluetooth => "Bluetooth MAC",
        }
    }
}

/// Basic device data exposed by the prototype.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Hardware identifier reported by the transport.
    pub identifier: String,
    /// Device serial number.
    pub serial_number: String,
    /// Wi-Fi address.
    pub wifi_mac: MacAddress,
    /// Bluetooth address.
    pub bluetooth_mac: MacAddress,
}

/// A fixed-size, length-prefixed record used by the prototype's binary parser.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyscfgRecord {
    /// Record payload, excluding the length byte.
    pub payload: Vec<u8>,
}

impl SyscfgRecord {
    /// Parses `[payload_length, payload...]`, rejecting trailing or missing bytes.
    pub fn parse(input: &[u8]) -> crate::Result<Self> {
        let Some((&length, payload)) = input.split_first() else {
            return Err(LitterPurpleError::MalformedData("record is empty".to_owned()));
        };
        if payload.len() != usize::from(length) {
            return Err(LitterPurpleError::MalformedData(format!(
                "record declares {length} bytes but contains {}",
                payload.len()
            )));
        }
        Ok(Self { payload: payload.to_vec() })
    }
}
