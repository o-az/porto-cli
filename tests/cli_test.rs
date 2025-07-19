use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("porto").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("CLI for Porto"));
}

#[test]
fn test_onboard_help() {
    let mut cmd = Command::cargo_bin("porto").unwrap();
    cmd.arg("onboard")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Create a Porto Account"));
}

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("porto").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("porto"));
}