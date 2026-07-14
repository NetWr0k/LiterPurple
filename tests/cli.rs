use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn mock_info_displays_device_data() {
    Command::new(env!("CARGO_BIN_EXE_litterpurple"))
        .args(["--mock", "info"])
        .assert()
        .success()
        .stdout(predicate::str::contains("C02LPURPLE01"));
}

#[test]
fn mock_write_confirms_verification() {
    Command::new(env!("CARGO_BIN_EXE_litterpurple"))
        .args(["--mock", "write", "serial", "C02-NEW123"])
        .assert()
        .success()
        .stdout(predicate::str::contains("updated and verified"));
}
