<img src = "test/liter.png"/>

# LitterPurple 💜

LitterPurple is a Windows first, open source Rust utility for reading and updating selected Apple SysCFG fields on supported A12/A13 devices **after** they have been placed in the required post-exploitation state by an external public research workflow.

It is inspired by older SysCFG tooling, but takes a small, typed, testable approach suitable for a modern Rust backend and a future WPF application.

> **Prototype status:** v0.1 provides a complete command shape, validation, mock transport, and public backend boundary. Physical USB communication is intentionally not implemented yet.

## Scope and safety

LitterPurple does not contain exploit logic, device payload logic, or hardware setup instructions. It assumes the operator is authorized to work on the device and that any required preparation was completed separately. The tool is intended for supported devices and legitimate servicing, research, and development use.

Writes are validated locally and then read back for verification. Keep backups and a documented recovery path before working with real hardware once USB support is added.

## Quick start

Install the current stable Rust toolchain, then run the safe in-memory demo:

```powershell
cargo run -- --mock detect
cargo run -- --mock info
cargo run -- --mock read
cargo run -- --mock write serial C02-LP-0001
cargo run -- --mock write wifi 02:11:22:33:44:55
cargo test
```

Without `--mock`, the v0.1 build reports that no prepared device is detected because the production USB transport has not been connected yet.

## CLI

```text
litterpurple detect
litterpurple info
litterpurple read
litterpurple write serial <serial>
litterpurple write wifi <mac>
litterpurple write bt <mac>
```

For the prototype, serials must contain 1–32 ASCII letters, digits, or hyphens. MAC addresses use six colon-separated hexadecimal octets.

## Project layout

```text
src/
  transport.rs   Device communication boundary and mock implementation
  service.rs     Read/write use cases plus readback verification
  model.rs       Strongly typed device and binary data models
  validation.rs  Input validation
  ffi.rs         Minimal C ABI starter surface
docs/            Architecture, API, and developer guides
```

## Contributing and license

See [CONTRIBUTING.md](CONTRIBUTING.md), the [developer setup guide](docs/developer-setup.md), and [architecture notes](docs/architecture.md). LitterPurple is released under the [MIT License](LICENSE).
