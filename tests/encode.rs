use assert_cmd::Command;

#[test]
fn encode_test_vectors_empty_rfc4648() {
    let mut cmd = Command::cargo_bin("b64").unwrap();
    cmd.arg("").assert().success().stdout("\n");
}

#[test]
fn encode_test_vectors_f_rfc4648() {
    let mut cmd = Command::cargo_bin("b64").unwrap();
    cmd.args(vec!["f"]).assert().success().stdout("Zg==\n");
}

#[test]
fn encode_test_vectors_fo_rfc4648() {
    let mut cmd = Command::cargo_bin("b64").unwrap();
    cmd.args(vec!["fo"]).assert().success().stdout("Zm8=\n");
}

#[test]
fn encode_test_vectors_foo_rfc4648() {
    let mut cmd = Command::cargo_bin("b64").unwrap();
    cmd.args(vec!["foo"]).assert().success().stdout("Zm9v\n");
}

#[test]
fn encode_test_vectors_foob_rfc4648() {
    let mut cmd = Command::cargo_bin("b64").unwrap();
    cmd.args(vec!["foob"]).assert().success().stdout("Zm9vYg==\n");
}

#[test]
fn encode_test_vectors_fooba_rfc4648() {
    let mut cmd = Command::cargo_bin("b64").unwrap();
    cmd.args(vec!["fooba"]).assert().success().stdout("Zm9vYmE=\n");
}

#[test]
fn encode_test_vectors_foobar_rfc4648() {
    let mut cmd = Command::cargo_bin("b64").unwrap();
    cmd.args(vec!["foobar"]).assert().success().stdout("Zm9vYmFy\n");
}
