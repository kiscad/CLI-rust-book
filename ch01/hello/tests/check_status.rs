use std::process::Command;

/**
 * test the return status of process
 */
#[test]
fn runs_ok() {
  let mut cmd = Command::new("ls");
  let res = cmd.output();
  assert!(res.is_ok());
}

#[test]
fn runs_fail() {
  let mut cmd = Command::new("hello");
  let res = cmd.output();
  assert!(res.is_err());
}

#[test]
fn run_assert_cmd() {
  let mut cmd = assert_cmd::Command::cargo_bin("hello").unwrap();
  cmd.assert().success();
}

#[test]
fn run_proces_abort() {
  let mut cmd = assert_cmd::Command::cargo_bin("abort").unwrap();
  cmd.assert().failure();
}
