use crate::alphabet::{Alphabet, Classic};

pub fn decode(bytes: &String) -> Vec<u8> {
    let alphabet = Classic {};
    decode_using_alphabet(alphabet, bytes)
}

pub fn decode_using_alphabet<T: Alphabet>(alphabet: T, data: &String) -> Vec<u8> {
    // if data is not multiple of four bytes, data is invalid
    if data.len() % 4 != 0 {
        panic!("Invalid data");
    }

    data
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| original(&alphabet, chunk) )
        .flat_map(stitch)
        .collect()
}

fn original<T: Alphabet>(alphabet: &T, chunk: &[char]) -> Vec<u8> {
    chunk
        .iter()
        .filter(|character| *character != &alphabet.get_padding_char())
        .map(|character| index_for_char(alphabet, &character))
        .collect()
}

fn stitch(bytes: Vec<u8>) -> Vec<u8> {
    let out = match bytes.len() {
        2 => vec![
            (bytes[0] & 0b00111111) << 2 | bytes[1] >> 4,
            (bytes[1] & 0b00001111) << 4,
        ],

        3 => vec![
            (bytes[0] & 0b00111111) << 2 | bytes[1] >> 4,
            (bytes[1] & 0b00001111) << 4 | bytes[2] >> 2,
            (bytes[2] & 0b00000011) << 6,
        ],

        4 => vec![
            (bytes[0] & 0b00111111) << 2 | bytes[1] >> 4,
            (bytes[1] & 0b00001111) << 4 | bytes[2] >> 2,
            (bytes[2] & 0b00000011) << 6 | bytes[3] & 0b00111111,
        ],

        _ => unimplemented!("number of bytes must be 2 - 4")
    };

    out.into_iter().filter(|&x| x > 0).collect()
}

fn index_for_char<T: Alphabet>(alphabet: &T, byte: &char) -> u8 {
    alphabet
        .get_index_for_char(*byte)
        .expect("unable to find character in alphabet")
}

#[cfg(test)]
mod tests {
    use super::decode;

    #[test]
    fn decode_one() {
        let encoded = String::from("YQ==");
        let expected = "a".as_bytes();
        assert_eq!(decode(&encoded), expected);
    }

    #[test]
    fn decode_two() {
        let encoded = String::from("YWI=");
        let expected = "ab".as_bytes();
        assert_eq!(decode(&encoded), expected);
    }

    #[test]
    fn decode_three() {
        let encoded = String::from("YWJj");
        let expected = "abc".as_bytes();
        assert_eq!(decode(&encoded), expected);
    }
}

