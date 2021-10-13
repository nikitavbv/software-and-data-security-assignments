use rand::thread_rng;
use rand::prelude::*;
use rayon::prelude::*;
use crate::wonderland::{ALPHABET, ENGLISH_DICTIONARY};

#[derive(Debug, Clone)]
pub struct SubstitutionKey {
    character_map: Vec<char>,
}

#[derive(Debug, Clone)]
pub struct PossibleSolution {
    key: SubstitutionKey,
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

impl PossibleSolution {

    pub fn score(&self, encoded_data: &str) -> usize {
        count_english_words(&self.key.decode(encoded_data))
    }

    pub fn with_random_change(&self) -> Self {
        let mut new_key = self.key.character_map.clone();
        
        let total_changes = rand::thread_rng().gen_range(0..10);
        for _ in 0..total_changes {
            let a = rand::thread_rng().gen_range(0..new_key.len());
            let b = rand::thread_rng().gen_range(0..new_key.len());
            let tmp = new_key.get(a).unwrap().clone();
            new_key[a] = new_key[b];
            new_key[b] = tmp;
        }

        Self {
            key: SubstitutionKey { 
                character_map: 
                new_key 
            }
        }
    }
}

pub fn guess_key(encoded_data: &str) {
    let generation_size = 10000;
    let top_solutions = 500;
    let random_solutions = 100;

    let mut solutions = Vec::new();
    for _ in 0..generation_size {
        solutions.push(PossibleSolution {
            key: SubstitutionKey::random(),
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
                key: SubstitutionKey::random(),
            });
        }

        while new_solutions.len() < generation_size {
            let random_element = best_solutions.choose(&mut rand::thread_rng()).unwrap().0.clone();
            new_solutions.push(random_element.with_random_change());
        }

        solutions = new_solutions;
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