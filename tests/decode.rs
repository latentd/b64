#![cfg(test)]
use b64::decode;

#[test]
fn decode_test_vectors_empty_rfc4648() {
    let input = "";
    let want = "";
    assert_eq!(decode(input.to_string()), want);
}

#[test]
fn decode_test_vectors_f_rfc4648() {
    let input = "Zg==";
    let want = "f";
    assert_eq!(decode(input.to_string()), want);
}

#[test]
fn decode_test_vectors_fo_rfc4648() {
    let input = "Zm8=";
    let want = "fo";
    assert_eq!(decode(input.to_string()), want);
}

#[test]
fn decode_test_vectors_foo_rfc4648() {
    let input = "Zm9v";
    let want = "foo";
    assert_eq!(decode(input.to_string()), want);
}

#[test]
fn decode_test_vectors_foob_rfc4648() {
    let input = "Zm9vYg==";
    let want = "foob";
    assert_eq!(decode(input.to_string()), want);
}

#[test]
fn decode_test_vectors_fooba_rfc4648() {
    let input = "Zm9vYmE=";
    let want = "fooba";
    assert_eq!(decode(input.to_string()), want);
}

#[test]
fn decode_test_vectors_foobar_rfc4648() {
    let input = "Zm9vYmFy";
    let want = "foobar";
    assert_eq!(decode(input.to_string()), want);
}
