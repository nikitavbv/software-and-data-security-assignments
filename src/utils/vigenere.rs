use rand::{Rng, prelude::SliceRandom};
use rayon::prelude::*;
use crate::utils::{substitution::count_english_words, wonderland::COMBINED_ALPHABET};

#[derive(Clone, Debug)]
pub struct VigenereKey {
    pub key: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct PossibleSolution {
    key: VigenereKey,
}

impl VigenereKey {

    pub fn new(key: Vec<usize>) -> Self {
        VigenereKey {
            key
        }
    }

    pub fn random() -> Self {
        Self::random_with_len(rand::thread_rng().gen_range(3..16))
    }

    pub fn random_with_len(key_len: usize) -> Self {
        let mut key = Vec::new();
        for _ in 0..key_len {
            key.push(rand::thread_rng().gen_range(0..COMBINED_ALPHABET.len()));
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
        let index = match COMBINED_ALPHABET.iter().position(|t| t == &uppercase_char) {
            Some(v) => v,
            None => {
                panic!("failed to find character in the alphabet: {}", c);
            }
        };

        let new_index = (index + key) % COMBINED_ALPHABET.len();
        if c.is_uppercase() {
            COMBINED_ALPHABET[new_index].to_ascii_uppercase()
        } else {
            COMBINED_ALPHABET[new_index].to_ascii_lowercase()
        }
    }

    fn decode_char(&self, encoded_c: char, index: usize) -> char {
        if encoded_c == ' ' {
            return encoded_c;
        }

        let key = *self.key.get(index % self.key.len()).unwrap();
        let uppercase_char = encoded_c.to_ascii_uppercase();
        let index = match COMBINED_ALPHABET.iter().position(|t| t == &uppercase_char) {
            Some(v) => v,
            None => {
                panic!("failed to find character in the alphabet: {}", encoded_c);
            }
        };

        let new_index = (COMBINED_ALPHABET.len() + index - key) % COMBINED_ALPHABET.len();
        if encoded_c.is_uppercase() {
            COMBINED_ALPHABET[new_index].to_ascii_uppercase()
        } else {
            COMBINED_ALPHABET[new_index].to_ascii_lowercase()
        }
    }
}

impl PossibleSolution {

    pub fn score(&self, encoded_data: &str) -> usize {
        count_english_words(&self.key.decode(encoded_data))
    }

    pub fn with_random_change(&self) -> Self {
        let mut new_key = self.key.key.clone();

        let index_to_change = rand::thread_rng().gen_range(0..new_key.len());
        new_key[index_to_change] = (new_key[index_to_change] + rand::thread_rng().gen_range(0..COMBINED_ALPHABET.len())) % COMBINED_ALPHABET.len();

        Self {
            key: VigenereKey {
                key: new_key,
            }
        }
    }
}

pub fn guess_key_length(encoded_data: &str) -> usize {
    let mut best_score = 0;
    let mut best_key_length = 0;

    for key_length in 1..100 {
        let mut coincidences = 0;
        for index in 0..encoded_data.chars().count() {
            let index_with_offset = (index + key_length) % encoded_data.chars().count();
            let index_char = encoded_data.chars().nth(index).unwrap();
            let offset_char = encoded_data.chars().nth(index_with_offset).unwrap();

            if index_char.to_ascii_lowercase() == offset_char.to_ascii_lowercase() {
                coincidences += 1;
            }
        }

        if coincidences > best_score {
            best_score = coincidences;
            best_key_length = key_length;
        }

        println!("key length: {}, coincidences: {}", key_length, coincidences);
    }

    best_key_length
}

pub fn guess_key(encoded_data: &str, key_length: usize) {
    let generation_size = 1000;
    let top_solutions = 50;
    let random_solutions = 10;

    let mut solutions = Vec::new();
    for _ in 0..generation_size {
        solutions.push(PossibleSolution {
            key: VigenereKey::random_with_len(key_length),
        });
    }

    loop {
        let mut scored_solutions: Vec<(PossibleSolution, usize)> = solutions.par_iter()
            .map(|solution| (solution.clone(), solution.score(encoded_data)))
            .collect();
     
        scored_solutions.sort_by(|a, b| b.1.cmp(&a.1));
        println!("best score: {}", scored_solutions.get(0).unwrap().1);
        println!("{}", scored_solutions.get(0).unwrap().0.key.decode(encoded_data));
    
        let best_solutions = &scored_solutions[0..top_solutions];
        let mut new_solutions = Vec::new();
        for i in 0..top_solutions {
            new_solutions.push(scored_solutions[i].0.clone());
        }

        for _ in 0..random_solutions {
            new_solutions.push(PossibleSolution {
                key: VigenereKey::random_with_len(key_length),
            });
        }

        while new_solutions.len() < generation_size {
            let random_element = best_solutions.choose(&mut rand::thread_rng()).unwrap().0.clone();
            new_solutions.push(random_element.with_random_change());
        }

        solutions = new_solutions;
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

    #[test]
    fn what_is_char() {
        assert_eq!('е', "Привет мир!".chars().nth(4).unwrap());
    }
}