# Developer setup

## Requirements

- Windows 11 x64
- Latest stable Rust via [rustup](https://rustup.rs/)
- Git
- Optional: Visual Studio 2022 Build Tools with the Desktop development with C++ workload, for native USB dependencies added later

The v0.1 mock workflow needs no hardware and no special driver.

## Build and verify

```powershell
git clone <your-fork-url>
cd litterpurple
cargo build
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo run -- --mock info
```

Set `RUST_LOG=debug` for diagnostic logs. Do not paste logs containing real device identifiers into public issues.

## Future WPF host

The Rust library is configured to produce a `cdylib`. A .NET 8 WPF application will call a deliberately small C ABI through P/Invoke, then present operations through MVVM view models. Keep the native backend free of UI concepts; the service and error model are the contract.

## Release checklist

1. Run formatting, clippy, and the full test suite.
2. Validate CLI behavior against `MockTransport`.
3. Review all changed user-facing error messages and documentation.
4. When hardware support exists, run opt-in integration tests on an authorized test device with a recovery plan.
5. Update `CHANGELOG.md`, version metadata, and release notes.
