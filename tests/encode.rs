#![cfg(test)]
use b64::encode;

#[test]
fn encode_test_vectors_empty_rfc4648() {
    let input = "";
    let want = "";
    assert_eq!(encode(input.to_string()), want);
}

#[test]
fn encode_test_vectors_f_rfc4648() {
    let input = "f";
    let want = "Zg==";
    assert_eq!(encode(input.to_string()), want);
}

#[test]
fn encode_test_vectors_fo_rfc4648() {
    let input = "fo";
    let want = "Zm8=";
    assert_eq!(encode(input.to_string()), want);
}

#[test]
fn encode_test_vectors_foo_rfc4648() {
    let input = "foo";
    let want = "Zm9v";
    assert_eq!(encode(input.to_string()), want);
}

#[test]
fn encode_test_vectors_foob_rfc4648() {
    let input = "foob";
    let want = "Zm9vYg==";
    assert_eq!(encode(input.to_string()), want);
}

#[test]
fn encode_test_vectors_fooba_rfc4648() {
    let input = "fooba";
    let want = "Zm9vYmE=";
    assert_eq!(encode(input.to_string()), want);
}

#[test]
fn encode_test_vectors_foobar_rfc4648() {
    let input = "foobar";
    let want = "Zm9vYmFy";
    assert_eq!(encode(input.to_string()), want);
}
