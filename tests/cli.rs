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
