# Architecture

## Design boundary

LitterPurple starts after an external workflow has placed an authorized, supported device into the required state. The application has no component for exploitation, payload delivery, or exploit hardware control.

```text
CLI / future WPF UI
        |
   DeviceService
   /      |      \
validation model  verification
        |
 SyscfgTransport trait
      /         \
MockTransport   Windows USB transport (future)
```

## Layers

`model` contains serializable, strongly typed values. `validation` provides deterministic input checks. `service` implements use cases and verifies every write through the transport. `transport` is the only hardware-facing boundary; its mock permits fast tests and frontend work without a device.

The CLI and future WPF app must use `DeviceService`, not access a transport directly. This keeps validation and readback verification consistent across user interfaces.

## Future Windows transport

The production implementation will live in a separate, narrow transport module. It should enumerate only expected interfaces, report actionable errors, use timeouts, validate all replies before parsing, and never let untrusted device data trigger a panic. Integration tests should target a mock or recorded protocol fixture; real-device tests remain opt-in.

## Extensibility

Additional SysCFG fields extend `SyscfgField`, model parsing, service validation, and transport mapping together. Backup/restore should use a versioned, integrity-checked document with explicit device compatibility metadata. Plugins, if introduced, should have a deliberately restricted interface and no direct write access outside the service layer.
