mod alphabet;
mod encoder;
mod decoder;

fn main() {
    println!("Hello, world!");
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

