use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command; // Run programs

#[test]
fn term_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("todo")?;
    cmd.arg("term");
    cmd.assert().success();
    Ok(())
}

#[test]
fn terminal_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("todo")?;
    cmd.arg("terminal");
    cmd.assert().success();
    Ok(())
}

#[test]
fn conky_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("todo")?;
    cmd.arg("conky");
    cmd.assert().success();
    Ok(())
}
