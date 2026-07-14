//! LitterPurple's reusable backend library.
//!
//! This crate deliberately contains no exploit logic. A caller must only use
//! a transport after the device has been prepared by an external workflow.

pub mod error;
pub mod ffi;
pub mod model;
pub mod service;
pub mod transport;
pub mod validation;

pub use error::{LitterPurpleError, Result};
pub use model::{DeviceInfo, MacAddress, SyscfgField};
pub use service::DeviceService;
