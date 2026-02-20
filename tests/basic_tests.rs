use assert_cmd::Command;

#[test]
fn test_help_command() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_soroban-debug"));
    cmd.arg("--help");
    cmd.assert().success();
}

#[test]
fn test_version_command() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_soroban-debug"));
    cmd.arg("--version");
    cmd.assert().success();
}

#[test]
fn test_upgrade_check_help_command() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_soroban-debug"));
    cmd.arg("upgrade-check").arg("--help");
    cmd.assert().success();
}
