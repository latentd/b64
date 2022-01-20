use clap::Parser;
use colored::*;

use b64;

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
    let args = Cli::parse();
    let result;
    if args.decode {
        result = b64::decode::decode(args.input).cyan()
    } else {
        result = b64::encode::encode(args.input).green()
    }
    println!("{}", result);
}
