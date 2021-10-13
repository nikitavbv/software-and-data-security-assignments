use rand::Rng;

use crate::wonderland::ALPHABET;

#[derive(Clone, Debug)]
pub struct VigenereKey {
    key: Vec<usize>,
}

impl VigenereKey {

    pub fn new(key: Vec<usize>) -> Self {
        VigenereKey {
            key
        }
    }

    pub fn random() -> Self {
        let key_len = rand::thread_rng().gen_range(3..16);
        let mut key = Vec::new();
        for _ in 0..key_len {
            key.push(rand::thread_rng().gen_range(0..ALPHABET.len()));
        }

        Self {
            key
        }
    }

    pub fn encode(&self, data: &str) -> String {
        let mut result = Vec::new();
        for i in 0..data.len() {
            result.push(self.encode_char(data.chars().nth(i).unwrap(), i));
        }
        result.iter().collect()
    }

    pub fn decode(&self, encoded_data: &str) -> String {
        let mut result = Vec::new();
        for i in 0..encoded_data.len() {
            result.push(self.decode_char(encoded_data.chars().nth(i).unwrap(), i));
        }
        result.iter().collect()
    }

    fn encode_char(&self, c: char, index: usize) -> char {
        if c == ' ' {
            return c;
        }

        let key = *self.key.get(index % self.key.len()).unwrap();
        let uppercase_char = c.to_ascii_uppercase();
        let index = match ALPHABET.iter().position(|t| t == &uppercase_char) {
            Some(v) => v,
            None => {
                panic!("failed to find character in the alphabet: {}", c);
            }
        };

        let new_index = (index + key) % ALPHABET.len();
        if c.is_uppercase() {
            ALPHABET[new_index].to_ascii_uppercase()
        } else {
            ALPHABET[new_index].to_ascii_lowercase()
        }
    }

    fn decode_char(&self, encoded_c: char, index: usize) -> char {
        if encoded_c == ' ' {
            return encoded_c;
        }

        let key = *self.key.get(index % self.key.len()).unwrap();
        let uppercase_char = encoded_c.to_ascii_uppercase();
        let index = match ALPHABET.iter().position(|t| t == &uppercase_char) {
            Some(v) => v,
            None => {
                panic!("failed to find character in the alphabet: {}", encoded_c);
            }
        };

        let new_index = (ALPHABET.len() + index - key) % ALPHABET.len();
        if encoded_c.is_uppercase() {
            ALPHABET[new_index].to_ascii_uppercase()
        } else {
            ALPHABET[new_index].to_ascii_lowercase()
        }
    }
}

pub fn guess_key_length(encoded_data: &str) {
    for key_length in 3..16 {
        let mut coincidences = 0;
        for index in 0..encoded_data.len() {
            let index_with_offset = (index + key_length) % encoded_data.len();
            if encoded_data.chars().nth(index).unwrap().to_ascii_lowercase() == 
                encoded_data.chars().nth(index_with_offset).unwrap().to_ascii_lowercase() {
                coincidences += 1;
            }
        }

        println!("key length: {}, coincidences: {}", key_length, coincidences);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        let key = VigenereKey::random();
        let encoded = key.encode("Hello my dear friend");
        let decoded = key.decode(&encoded);
        assert_eq!(decoded, "Hello my dear friend");
    }
}