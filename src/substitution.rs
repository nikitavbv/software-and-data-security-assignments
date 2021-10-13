use rand::thread_rng;
use rand::prelude::*;
use crate::wonderland::ALPHABET;

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
}