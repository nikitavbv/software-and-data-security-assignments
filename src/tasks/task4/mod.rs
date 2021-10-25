use std::fs;
use rayon::prelude::*;
use rand::prelude::*;
use crate::utils::dictionaries::{ENGLISH_ALPHABET, count_english_words};

use log::{info, debug};

pub fn run() {
    let input = fs::read_to_string("tasks/task4/input").unwrap().replace("\n", "");
    let input = base64::decode(&input).unwrap();

    let key_length = guess_key_length(&input);
    info!("guessed key length as: {}", key_length);

    let result = guess_key(&input, key_length);
    fs::write("tasks/task4/output", result).unwrap();
}

#[derive(Clone, Debug)]
pub struct VigenereKey {
    pub key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct PossibleSolution {
    key: VigenereKey,
}

impl VigenereKey {

    pub fn new(key: Vec<u8>) -> Self {
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
            key.push(rand::thread_rng().gen());
        }

        Self {
            key
        }
    }

    pub fn encode(&self, data: &str) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        for i in 0..data.len() {
            result.push(self.encode_char(data.chars().nth(i).unwrap(), i));
        }
        result
    }

    pub fn decode(&self, encoded_data: &[u8]) -> String {
        let mut result = Vec::new();
        for i in 0..encoded_data.len() {
            result.push(self.decode_char(encoded_data[i], i));
        }
        result.iter().collect()
    }

    fn encode_char(&self, c: char, index: usize) -> u8 {
        let key = *self.key.get(index % self.key.len()).unwrap();
        (c as u8) ^ key
    }

    fn decode_char(&self, encoded_c: u8, index: usize) -> char {
        let key = *self.key.get(index % self.key.len()).unwrap();
        (encoded_c ^ key) as char
    }
}

impl PossibleSolution {

    pub fn score(&self, encoded_data: &[u8]) -> usize {
        count_english_words(&self.key.decode(encoded_data))
    }

    pub fn with_random_change(&self) -> Self {
        let mut new_key = self.key.key.clone();

        let index_to_change = rand::thread_rng().gen_range(0..new_key.len());
        new_key[index_to_change] = rand::thread_rng().gen();

        Self {
            key: VigenereKey {
                key: new_key,
            }
        }
    }
}

pub fn guess_key_length(encoded_data: &[u8]) -> usize {
    let mut best_score = 0;
    let mut best_key_length = 0;

    for key_length in 1..100 {
        let mut coincidences = 0;
        for index in 0..encoded_data.len() {
            let index_with_offset = (index + key_length) % encoded_data.len();
            let index_char = encoded_data[index];
            let offset_char = encoded_data[index_with_offset];

            if index_char.to_ascii_lowercase() == offset_char.to_ascii_lowercase() {
                coincidences += 1;
            }
        }

        if coincidences > best_score {
            best_score = coincidences;
            best_key_length = key_length;
        }

        debug!("key length: {}, coincidences: {}", key_length, coincidences);
    }

    best_key_length
}

pub fn guess_key(encoded_data: &[u8], key_length: usize) -> String {
    let generation_size = 1000;
    let top_solutions = 50;
    let random_solutions = 10;

    let mut solutions = Vec::new();
    for _ in 0..generation_size {
        solutions.push(PossibleSolution {
            key: VigenereKey::random_with_len(key_length),
        });
    }

    let mut current_iteration = 0;
    let mut highscore_iteration = 0;
    let mut best_score = 0;
    let mut best_solution = None;

    while current_iteration - highscore_iteration < 512 {
        current_iteration += 1;
        let mut scored_solutions: Vec<(PossibleSolution, usize)> = solutions.par_iter()
            .map(|solution| (solution.clone(), solution.score(encoded_data)))
            .collect();

        scored_solutions.sort_by(|a, b| b.1.cmp(&a.1));
        let score = scored_solutions.get(0).unwrap().1;
        if score > best_score {
            best_score = score;
            highscore_iteration = current_iteration;
            best_solution = Some(scored_solutions.get(0).unwrap().0.key.decode(encoded_data));
            info!("best solution yet: {}", best_solution.as_ref().unwrap());
        }

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

    best_solution.expect("Failed to find a solution")
}