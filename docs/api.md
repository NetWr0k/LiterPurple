# Backend API

The `litterpurple` crate is the backend contract for the CLI and future .NET desktop application.

## Rust surface

- `DeviceService<T>`: high-level operations—detect, `info`, `read_all`, and verified writes.
- `SyscfgTransport`: trait implemented by hardware transports and `MockTransport`.
- `DeviceInfo`, `MacAddress`, and `SyscfgField`: typed domain models.
- `validate_serial` and `validate_mac`: pure input-validation helpers.
- `LitterPurpleError`: errors designed for display without leaking low-level details.

Use `MockTransport` for application development and tests. `UnavailableUsbTransport` is the explicit v0.1 fallback; it returns `DeviceNotDetected` rather than pretending hardware support exists.

## C ABI (prototype)

The built `cdylib` exposes:

```c
uint32_t litterpurple_version(void);
```

It currently returns `1` for version 0.1. The ABI is intentionally minimal while ownership, error, and string conventions are designed. Future C-facing functions must document caller ownership and must not accept unchecked pointers.

## Error behavior

Callers should map `LitterPurpleError` to clear UI text. A failed write is only successful after its readback matches the requested value. Invalid input is rejected before a transport call.
