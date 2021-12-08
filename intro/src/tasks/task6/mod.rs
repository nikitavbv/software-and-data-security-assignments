use std::fs;
use log::info;
use rand::prelude::*;
use utils::dictionaries::ENGLISH_ALPHABET;
use utils::{CombinedSolutionScorer, GeneticAlgorithmRunner, GeneticAlgorithmSolution, GeneticAlgorithmSolutionProducer, NGramSolutionScorer, ScoringFactor, SimpleGeneticAlgorithmRunner};
use crate::tasks::task4::guess_key_length;

#[derive(Debug, Clone)]
struct Task6Solution {
    key: Vec<[char; 26]>,
}

struct Task6SolutionProducer {
    key_length: usize,
}

impl GeneticAlgorithmSolution for Task6Solution {
    type OriginalType = String;
    type EncodedType = String;

    fn decode(&self, encoded_data: &Self::EncodedType) -> Self::OriginalType {
        let mut output = Vec::new();
        for i in 0..encoded_data.chars().count() {
            let encoded_char = encoded_data.chars().nth(i).unwrap();
            let key = &self.key[i % self.key.len()];
            let key_index = ENGLISH_ALPHABET.iter().position(|c| c == &encoded_char).unwrap();
            output.push(key[key_index].to_lowercase().to_string());
        }

        output.into_iter().collect()
    }

    fn randomly_change(&self) -> Self {
        let key_to_change = (0..self.key.len()).choose(&mut rand::thread_rng()).unwrap();
        let index_to_change = (0..self.key[key_to_change].len()).choose(&mut rand::thread_rng()).unwrap();
        let index_to_swap = (0..self.key[key_to_change].len()).choose(&mut rand::thread_rng()).unwrap();

        let mut key = self.key.clone();

        let t = self.key[key_to_change][index_to_change];
        key[key_to_change][index_to_change] = key[key_to_change][index_to_swap];
        key[key_to_change][index_to_swap] = t;

        Self {
            key,
        }
    }

    fn randomly_combine(&self, other: &Self) -> Self {
        let mut key = self.key.clone();
        let key_to_change = (0..self.key.len()).choose(&mut rand::thread_rng()).unwrap();
        key[key_to_change] = other.key[key_to_change];

        Self {
            key,
        }
    }
}

impl GeneticAlgorithmSolutionProducer<Task6Solution> for Task6SolutionProducer {
    fn random(&self) -> Task6Solution {
        Task6Solution {
            key: (0..self.key_length).map(|_| {
                let mut alphabet = ENGLISH_ALPHABET.clone();
                alphabet.shuffle(&mut rand::thread_rng());
                alphabet
            }).collect(),
        }
    }
}

pub fn run() {
    let input = fs::read_to_string("tasks/task6/input").unwrap().replace("\n", "");

    let key_length = guess_key_length(input.as_bytes());
    info!("key length is: {}", key_length);

    let key_length = 4; // instead of 16 picked automatically by default

    let producer = Task6SolutionProducer {
        key_length,
    };
    let scorer = NGramSolutionScorer::new(3);

    let mut runner = SimpleGeneticAlgorithmRunner::new(
        producer,
        scorer,
        input.clone(),
    );

    let mut best_iteration_score = f32::MIN;

    loop {
        runner.run_iteration();

        let (solution, score) = runner.best_solution();
        if *score > best_iteration_score {
            let decoded = solution.decode(&input);
            info!("Best solution so far: {:?} {} {}", solution, decoded, score);
            best_iteration_score = *score;

            fs::write("tasks/task6/output", decoded);
        }
    }
}