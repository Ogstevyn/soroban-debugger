use assert_cmd::Command;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("soroban-debug").unwrap();
    cmd.arg("--help");
    cmd.assert().success();
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("soroban-debug").unwrap();
    cmd.arg("--version");
    cmd.assert().success();
}

// More tests will be added as functionality is implemented