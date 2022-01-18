use clap::Parser;
use human_panic::setup_panic;

const INDEX_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z', '0', '1', '2', '3',
    '4', '5', '6', '7', '8', '9', '+', '-',
];

fn get_sextets(octets: &[u8]) -> Vec<u8> {

    let mut sextets: Vec<u8> = Vec::new();

    for i in 0..octets.len() {
        let n = i % 3; 
        if n == 0 {
            sextets.push(
                (octets[i] & 0b11111100) >> 2
            );
            if i == octets.len() - 1 {
                sextets.push(
                    (octets[i] & 0b00000011) << 4
                )
            }
        } else if n == 1 {
            sextets.push(
                (octets[i-1] & 0b00000011) << 4 | ((octets[i] & 0b11110000) >> 4)
            );
            if i == octets.len() - 1 {
                sextets.push(
                    (octets[i] & 0b00001111) << 2
                )
            }
        } else if n == 2 {
            sextets.push(
                (octets[i-1] & 0b00001111) << 2 | ((octets[i] & 0b11000000) >> 6)
            );
            sextets.push(octets[i] & 0b00111111)
        };
    };
    sextets
}

fn encode(input: String) -> String {
    let mut encoded = String::from("");

    let input_bytes = input.as_bytes();
    let sextets = get_sextets(input_bytes); 
    for s in sextets {
        encoded.push(INDEX_TABLE[s as usize]);
    };
    for _ in encoded.len()%4..4 {
        encoded.push('=');
    };
    encoded
}

/// Base64 encode/decode util
#[derive(Parser)]
struct Cli {
    /// Input text
    input: String,
}

fn main() {
    setup_panic!();

    let args = Cli::parse();
    println!("{}", encode(args.input));
    panic!("tet")
}
