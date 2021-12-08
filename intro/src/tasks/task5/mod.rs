use std::fs;
use rand::prelude::*;
use log::info;
use utils::{
    GeneticAlgorithmRunner,
    GeneticAlgorithmSolution,
    GeneticAlgorithmSolutionProducer,
    NGramSolutionScorer,
    SimpleGeneticAlgorithmRunner,
    dictionaries::ENGLISH_ALPHABET,
};

#[derive(Debug, Clone)]
struct Task5Solution {
    key: [char; 26],
}

struct Task5SolutionProducer {
}

impl GeneticAlgorithmSolution for Task5Solution {
    type OriginalType = String;
    type EncodedType = String;

    fn decode(&self, encoded_data: &Self::EncodedType) -> Self::OriginalType {
        encoded_data.chars()
            .map(|chr| ENGLISH_ALPHABET.iter().position(|c| c == &chr).unwrap())
            .map(|i| self.key[i].to_lowercase().to_string())
            .collect()
    }

    fn randomly_change(&self) -> Self {
        let mut key = self.key.clone();
        let change_index = (0..key.len()).choose(&mut rand::thread_rng()).unwrap();
        let swap_with_index = (0..key.len()).choose(&mut rand::thread_rng()).unwrap();

        let tmp = key[change_index];
        key[change_index] = key[swap_with_index];
        key[swap_with_index] = tmp;

        Self {
            key,
        }
    }

    fn randomly_combine(&self, other: &Self) -> Self {
        // let's keep it simple and do not combine
        self.clone()
    }
}

impl GeneticAlgorithmSolutionProducer<Task5Solution> for Task5SolutionProducer {
    fn random(&self) -> Task5Solution {
        let mut key = ENGLISH_ALPHABET.clone();
        key.shuffle(&mut rand::thread_rng());

        Task5Solution {
            key,
        }
    }
}

pub fn run() {
    let input = fs::read_to_string("tasks/task5/input").unwrap().replace("\n", "");

    let producer = Task5SolutionProducer {};
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

            fs::write("tasks/task5/output", decoded).unwrap();
        }
    }
}