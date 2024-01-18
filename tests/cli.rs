use assert_cmd::Command;

#[test]
fn woks() {
    assert!(true)
}

#[test]
fn runs_error() {
    let mut cmd = Command::new("hello");
    let res = cmd.output();
    assert!(res.is_err())
}

#[test]
fn runs_success() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok())
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}
#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("practice-rust").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn hello_world() {
    let mut cmd = Command::cargo_bin("practice-rust").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}
