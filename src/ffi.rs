//! Minimal ABI-stable entry points for a future .NET P/Invoke layer.

#![allow(unsafe_code)]

/// Returns the major/minor/patch version as `0x00MMmmpp`.
// SAFETY: This intentionally exposes a stable symbol to native callers. It
// accepts no pointers, reads no memory, and has no side effects.
#[unsafe(no_mangle)]
pub extern "C" fn litterpurple_version() -> u32 {
    0x0000_0001
}
