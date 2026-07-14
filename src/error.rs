//! Error types with messages suitable for command-line users.

use thiserror::Error;

/// A project-wide result alias.
pub type Result<T> = std::result::Result<T, LitterPurpleError>;

/// Errors produced while interacting with a prepared device.
#[derive(Debug, Error)]
pub enum LitterPurpleError {
    /// No compatible device was available to the selected transport.
    #[error("No prepared device was detected. Connect a supported device that has already completed the external preparation workflow.")]
    DeviceNotDetected,

    /// The connected device does not meet this build's support policy.
    #[error("Unsupported device: {0}")]
    UnsupportedDevice(String),

    /// A future USB transport could not complete communication.
    #[error("USB communication failed: {0}")]
    UsbCommunication(String),

    /// Serial input did not meet the current conservative validation rules.
    #[error("Invalid serial number: {0}")]
    InvalidSerial(String),

    /// MAC input did not meet the expected six-octet format.
    #[error("Invalid MAC address: {0}")]
    InvalidMac(String),

    /// A field could not be read.
    #[error("Could not read {field}: {reason}")]
    ReadFailure {
        /// Field name.
        field: String,
        /// Human-readable cause.
        reason: String,
    },

    /// A field could not be written.
    #[error("Could not write {field}: {reason}")]
    WriteFailure {
        /// Field name.
        field: String,
        /// Human-readable cause.
        reason: String,
    },

    /// A post-write readback did not match the requested value.
    #[error("Verification failed for {field}: expected {expected}, received {actual}")]
    VerificationFailure {
        /// Field name.
        field: String,
        /// Requested value.
        expected: String,
        /// Read-back value.
        actual: String,
    },

    /// Binary data does not match the expected structure.
    #[error("Malformed SysCFG data: {0}")]
    MalformedData(String),
}
