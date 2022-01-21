use clap::{App, Arg};
use colored::*;

use b64;

#[derive(Debug)]
struct Config {
    input: String,
    decode: bool,
}

fn get_args() -> b64::B64Result<Config> {
    let matches = App::new("b64")
        .version("0.0.7")
        .about("CLI tool to encode/decode base64")
        .arg(
            Arg::new("input")
                .value_name("INPUT")
                .help("Input text")
                .default_value("-"),
        )
        .arg(
            Arg::new("decode")
                .short('d')
                .long("decode")
                .help("Decode mode")
                .takes_value(false),
        )
        .get_matches();
    
    Ok(Config {
        input: matches.value_of("input").unwrap().to_string(),
        decode: matches.is_present("decode"),
    })
}

fn run(config: Config) -> b64::B64Result<()> {
    let result;
    match config.decode {
        true => result = b64::decode(config.input)?.cyan(),
        false => result = b64::encode(config.input)?.green(),
    }
    println!("{}", result);
    Ok(())
}

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
