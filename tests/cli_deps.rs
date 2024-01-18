use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("practice-rust").unwrap();
    cmd.assert().success();
}
