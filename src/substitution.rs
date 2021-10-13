use std::ascii::AsciiExt;

use rand::thread_rng;
use rand::prelude::*;
use crate::wonderland::{ALPHABET, ENGLISH_DICTIONARY};

pub struct SubstitutionKey {
    character_map: Vec<char>,
}

impl SubstitutionKey {

    pub fn random() -> Self {
        let mut character_map = ALPHABET.clone();
        character_map.shuffle(&mut thread_rng());

        Self {
            character_map,
        }
    }

    pub fn encode(&self, data: &str) -> String {
        let mut result = Vec::new();
        for char in data.chars() {
            result.push(self.encode_char(char));
        }
        result.iter().collect()
    }

    pub fn decode(&self, encoded_data: &str) -> String {
        let mut result = Vec::new();
        for char in encoded_data.chars() {
            result.push(self.decode_char(char));
        }
        result.iter().collect()
    }

    fn encode_char(&self, c: char) -> char {
        if c == ' ' {
            return c;
        }

        let uppercase_char = c.to_ascii_uppercase();
        let index = match ALPHABET.iter().position(|t| t == &uppercase_char) {
            Some(v) => v,
            None => {
                panic!("failed to find character in the alphabet: {}", c);
            }
        };
        let result = *self.character_map.get(index).unwrap();

        if c.is_uppercase() {
            result.to_ascii_uppercase()
        } else {
            result.to_ascii_lowercase()
        }
    }

    fn decode_char(&self, encoded_c: char) -> char {
        if encoded_c == ' ' {
            return encoded_c;
        }

        let uppercase_char = encoded_c.to_ascii_uppercase();
        let index = self.character_map.iter().position(|t| t == &uppercase_char).unwrap();
        let result = *ALPHABET.get(index).unwrap();

        if encoded_c.is_uppercase() {
            result.to_ascii_uppercase()
        } else {
            result.to_ascii_lowercase()
        }
    }
}

fn count_english_words(sentence: &str) -> usize {
    sentence.split(" ").filter(|word| is_english_word(word)).count()
}

fn is_english_word(word: &str) -> bool {
    ENGLISH_DICTIONARY.contains(&word.to_ascii_lowercase())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn cookie_is_a_word() {
        assert!(is_english_word("cookie"))
    }

    #[test]
    fn taalei_is_not_a_word() {
        assert!(!is_english_word("taalei"));
    }

    #[test]
    fn count_words_in_sentence() {
        assert_eq!(count_english_words("Hello my dear friend do you want a taalei"), 8)
    }

    #[test]
    fn encode_decode() {
        let key = SubstitutionKey::random();
        let encoded = key.encode("HeLlo my DeAr frIeNd");
        assert_eq!(key.decode(&encoded), "HeLlo my DeAr frIeNd");
    }
}