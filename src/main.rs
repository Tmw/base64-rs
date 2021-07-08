mod alphabet;
mod encoder;
mod decoder;

use std::io::{self, Read};

fn error(message: &str) {
    println!("Error: {}", message);
    std::process::exit(1);
}

fn main() -> io::Result<()> {
    if std::env::args().count() < 2 {
        error("Too little arguments");
    }

    match std::env::args().nth(1) {
        Some(subcommand) => parse_input(subcommand),
        None => error("Subcommand not found"),
    }

    Ok(())
}

fn parse_input(subcommand: String) {
    match subcommand.as_str() {
        "encode" => encode(),
        "decode" => decode(),
        subcommand => {
            let error_message = format!("subcommand {} not recognized", subcommand);
            error(error_message.as_str());
        },
    }
}

fn encode() {
    let mut input = String::new();
    match io::stdin().read_to_string(&mut input) {
        Ok(_) => {
            let encoded = encoder::encode(input.trim().as_bytes());
            print!("{}", encoded);
        },

        Err(_) => error("Unable to read STDIN"),
    }
}

fn decode() {
    let mut input = String::new();
    match io::stdin().read_to_string(&mut input) {
        Ok(_) => {
            let decoded = decoder::decode(&input.trim().to_owned());
            let decoded_as_string = std::str::from_utf8(&decoded).unwrap();
            print!("{}", decoded_as_string);
        },

        Err(_) => error("Unable to read STDIN"),
    }
}

#[cfg(test)]
mod tests {
    use super::decoder::decode;
    use super::encoder::encode;

    #[test]
    fn test_roundtrip() {
        let input_str = "and there we are again. Back and forth, through the algoritm!";
        let input_data = input_str.as_bytes();
        let encoded = encode(input_data);
        let decoded = decode(&encoded);
        assert_eq!(decoded, input_data);
    }
}

