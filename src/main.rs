use clap::Parser;
use colored::*;
use human_panic::setup_panic;

const INDEX_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '-',
];

fn convert_to_sextets(input: String) -> Vec<u8> {
    let octets = input.as_bytes();
    let mut sextets: Vec<u8> = Vec::new();

    if input.len() == 0 {
        return sextets;
    };

    let mut i = 0;
    loop {
        match i % 3 {
            0 => sextets.push((octets[i] & 0b11111100) >> 2),
            1 => sextets.push((octets[i - 1] & 0b00000011) << 4 | ((octets[i] & 0b11110000) >> 4)),
            2 => {
                sextets.push((octets[i - 1] & 0b00001111) << 2 | ((octets[i] & 0b11000000) >> 6));
                sextets.push(octets[i] & 0b00111111);
            }
            _ => {}
        };
        if i >= octets.len() - 1 {
            break;
        };
        i += 1;
    }
    match i % 3 {
        0 => sextets.push((octets[i] & 0b00000011) << 4),
        1 => sextets.push((octets[i] & 0b00001111) << 2),
        _ => (),
    };

    sextets
}

fn add_padding(encoded: &mut String) {
    let length = encoded.len();
    if length % 4 != 0 {
        for _ in (length % 4)..4 {
            encoded.push('=');
        }
    };
}

fn render_sextets(sextets: Vec<u8>) -> String {
    let mut encoded = String::new();
    for s in sextets {
        encoded.push(INDEX_TABLE[s as usize]);
    }
    encoded
}

fn base64_encode(input: String) -> String {
    let sextets = convert_to_sextets(input);
    let mut encoded = render_sextets(sextets);
    add_padding(&mut encoded);
    encoded
}

#[test]
fn encode_test_vectors_empty_rfc4648() {
    let input = "";
    let want = "";
    assert_eq!(base64_encode(input.to_string()), want);
}

#[test]
fn encode_test_vectors_f_rfc4648() {
    let input = "f";
    let want = "Zg==";
    assert_eq!(base64_encode(input.to_string()), want);
}

#[test]
fn encode_test_vectors_fo_rfc4648() {
    let input = "fo";
    let want = "Zm8=";
    assert_eq!(base64_encode(input.to_string()), want);
}

#[test]
fn encode_test_vectors_foo_rfc4648() {
    let input = "foo";
    let want = "Zm9v";
    assert_eq!(base64_encode(input.to_string()), want);
}

#[test]
fn encode_test_vectors_foob_rfc4648() {
    let input = "foob";
    let want = "Zm9vYg==";
    assert_eq!(base64_encode(input.to_string()), want);
}

#[test]
fn encode_test_vectors_fooba_rfc4648() {
    let input = "fooba";
    let want = "Zm9vYmE=";
    assert_eq!(base64_encode(input.to_string()), want);
}

#[test]
fn encode_test_vectors_foobar_rfc4648() {
    let input = "foobar";
    let want = "Zm9vYmFy";
    assert_eq!(base64_encode(input.to_string()), want);
}

fn strip_padding(input: String) -> String {
    input.trim_end_matches("=").to_string()
}

fn reverse_render_sextets(text: String) -> Vec<u8> {
    text.chars()
        .map(|c| INDEX_TABLE.iter().position(|v| v == &c).unwrap())
        .map(|c| c as u8)
        .collect::<Vec<u8>>()
}

fn reverse_convert_to_sextets(sextets: Vec<u8>) -> String {
    let mut decoded = String::new();
    for chunk in sextets.chunks(4).collect::<Vec<&[u8]>>() {
        let length = chunk.len();
        let mut bits: u32 = 0;
        for i in 0..length {
            bits |= (chunk[i] as u32) << (3 - i) * 6;
        }
        let bytes = bits.to_be_bytes()[1..length].to_vec();
        decoded.push_str(std::str::from_utf8(&bytes).unwrap());
    }
    decoded
}

fn base64_decode(input: String) -> String {
    let stripped = strip_padding(input);
    let sextets = reverse_render_sextets(stripped);
    reverse_convert_to_sextets(sextets)
}

#[test]
fn decode_test_vectors_empty_rfc4648() {
    let input = "";
    let want = "";
    assert_eq!(base64_decode(input.to_string()), want);
}

#[test]
fn decode_test_vectors_f_rfc4648() {
    let input = "Zg==";
    let want = "f";
    assert_eq!(base64_decode(input.to_string()), want);
}

#[test]
fn decode_test_vectors_fo_rfc4648() {
    let input = "Zm8=";
    let want = "fo";
    assert_eq!(base64_decode(input.to_string()), want);
}

#[test]
fn decode_test_vectors_foo_rfc4648() {
    let input = "Zm9v";
    let want = "foo";
    assert_eq!(base64_decode(input.to_string()), want);
}

#[test]
fn decode_test_vectors_foob_rfc4648() {
    let input = "Zm9vYg==";
    let want = "foob";
    assert_eq!(base64_decode(input.to_string()), want);
}

#[test]
fn decode_test_vectors_fooba_rfc4648() {
    let input = "Zm9vYmE=";
    let want = "fooba";
    assert_eq!(base64_decode(input.to_string()), want);
}

#[test]
fn decode_test_vectors_foobar_rfc4648() {
    let input = "Zm9vYmFy";
    let want = "foobar";
    assert_eq!(base64_decode(input.to_string()), want);
}

/// Base64 encode/decode util
#[derive(Parser)]
struct Cli {
    /// Input text
    input: String,
    /// Decode Mode
    #[clap(short, long)]
    decode: bool,
}

fn main() {
    setup_panic!();

    let args = Cli::parse();
    let result;
    if args.decode {
        result = base64_decode(args.input).cyan()
    } else {
        result = base64_encode(args.input).green()
    }
    println!("{}", result);
}
