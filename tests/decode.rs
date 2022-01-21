use assert_cmd::Command;

#[test]
fn decode_test_vectors_empty_rfc4648() {
    let mut cmd = Command::cargo_bin("b64").unwrap();
    cmd.arg("-d").assert().success().stdout("\n");
}

#[test]
fn decode_test_vectors_f_rfc4648() {
    let mut cmd = Command::cargo_bin("b64").unwrap();
    cmd.args(vec!["-d", "Zg=="]).assert().success().stdout("f\n");
}

#[test]
fn decode_test_vectors_fo_rfc4648() {
    let mut cmd = Command::cargo_bin("b64").unwrap();
    cmd.args(vec!["-d", "Zm8="]).assert().success().stdout("fo\n");
}

#[test]
fn decode_test_vectors_foo_rfc4648() {
    let mut cmd = Command::cargo_bin("b64").unwrap();
    cmd.args(vec!["-d", "Zm9v"]).assert().success().stdout("foo\n");
}

#[test]
fn decode_test_vectors_foob_rfc4648() {
    let mut cmd = Command::cargo_bin("b64").unwrap();
    cmd.args(vec!["-d", "Zm9vYg=="]).assert().success().stdout("foob\n");
}

#[test]
fn decode_test_vectors_fooba_rfc4648() {
    let mut cmd = Command::cargo_bin("b64").unwrap();
    cmd.args(vec!["-d", "Zm9vYmE="]).assert().success().stdout("fooba\n");
}

#[test]
fn decode_test_vectors_foobar_rfc4648() {
    let mut cmd = Command::cargo_bin("b64").unwrap();
    cmd.args(vec!["-d", "Zm9vYmFy"]).assert().success().stdout("foobar\n");
}
