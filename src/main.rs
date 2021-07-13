mod alphabet;
mod decoder;
mod encoder;

use std::fmt;
use std::io::{self, Read};

enum CLIError {
    TooLittleArguments,
    InvalidSubcommand(String),
    StdInUnreadable,
    DecodingError,
}

impl std::fmt::Debug for CLIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::TooLittleArguments => write!(f, "Too little arguments provided"),

            Self::InvalidSubcommand(cmd) => write!(f, "Invalid subcommand provided: \"{}\"", cmd),

            Self::StdInUnreadable => write!(f, "Unable to read STDIN"),

            Self::DecodingError => write!(f, "An error occured while decoding the data"),
        }
    }
}

fn main() -> Result<(), CLIError> {
    if std::env::args().count() < 2 {
        return Err(CLIError::TooLittleArguments);
    }

    let subcommand = std::env::args()
        .nth(1)
        .ok_or_else(|| CLIError::TooLittleArguments)?;

    let input = read_stdin()?;

    let output = match subcommand.as_str() {
        "encode" => Ok(encode(&input)),
        "decode" => Ok(decode(&input)?),
        cmd => Err(CLIError::InvalidSubcommand(cmd.to_string())),
    }?;

    print!("{}", output);

    Ok(())
}

fn read_stdin() -> Result<String, CLIError> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|_| CLIError::StdInUnreadable)?;

    Ok(input.trim().to_string())
}

fn encode(input: &String) -> String {
    encoder::encode(input.as_bytes())
}

fn decode(input: &String) -> Result<String, CLIError> {
    let decoded = decoder::decode(input).map_err(|_| CLIError::DecodingError)?;

    let decoded_as_string = std::str::from_utf8(&decoded)
        .map_err(|_| CLIError::DecodingError)?;

    Ok(decoded_as_string.to_owned())
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
        assert!(decoded.is_ok());
        assert_eq!(decoded.unwrap(), input_data);
    }
}
