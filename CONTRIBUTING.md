# Contributing to LitterPurple

Thanks for helping make LitterPurple clear, dependable, and welcoming to maintain.

## Before you start

Please open an issue or start a discussion for significant architectural changes. Keep proposed work in scope: the project communicates with already-prepared devices; it does not implement exploit workflows or hardware preparation.

## Development loop

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo run -- --mock info
```

## Pull requests

- Keep changes focused and explain the user-facing outcome.
- Add or update tests for behavior changes.
- Keep modules small; put USB-specific work behind `SyscfgTransport`.
- Avoid `unsafe`. If an exception is unavoidable, document why and constrain it tightly.
- Do not add real-device identifiers, backups, credentials, or private research material to the repository.
- Update documentation and `CHANGELOG.md` when appropriate.

## Style

Use stable Rust and `rustfmt`. Favor explicit types, clear errors, and small public APIs with rustdoc. Treat invalid device data and failed verification as normal error paths, not panics.

## Reporting issues

Include the LitterPurple version, Windows version, command used, and a redacted log. Never publish personal serial numbers, MAC addresses, or device backups.
