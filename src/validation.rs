//! Input validation kept separate from hardware access.

use crate::error::{LitterPurpleError, Result};

/// Validates a conservative serial-number format for this prototype.
///
/// Accepted input is 1–32 ASCII letters, digits, or hyphens. Exact device
/// family rules can be added when the production transport is implemented.
pub fn validate_serial(value: &str) -> Result<()> {
    let valid_length = (1..=32).contains(&value.len());
    let valid_characters = value
        .bytes()
        .all(|character| character.is_ascii_alphanumeric() || character == b'-');
    if valid_length && valid_characters {
        Ok(())
    } else {
        Err(LitterPurpleError::InvalidSerial(
            "use 1–32 ASCII letters, digits, or hyphens".to_owned(),
        ))
    }
}

/// Validates a colon-delimited six-octet MAC address.
pub fn validate_mac(value: &str) -> Result<()> {
    let parts: Vec<_> = value.split(':').collect();
    let valid = parts.len() == 6
        && parts.iter().all(|part| {
            part.len() == 2 && part.bytes().all(|character| character.is_ascii_hexdigit())
        });
    if valid {
        Ok(())
    } else {
        Err(LitterPurpleError::InvalidMac(
            "use six hexadecimal octets, for example 02:11:22:33:44:55".to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_canonical_mac() {
        assert!(validate_mac("02:11:22:aa:BB:ff").is_ok());
    }

    #[test]
    fn rejects_short_mac() {
        assert!(validate_mac("02:11:22:33:44").is_err());
    }

    #[test]
    fn accepts_conservative_serial() {
        assert!(validate_serial("C02-TEST123").is_ok());
    }

    #[test]
    fn rejects_serial_with_spaces() {
        assert!(validate_serial("C02 TEST").is_err());
    }
}
