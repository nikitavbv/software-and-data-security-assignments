use std::fs;
use env_logger::Env;
use log::info;
use rand::prelude::*;
use rand::distributions::Uniform;
use utils::{GeneticAlgorithmSolution, GeneticAlgorithmSolutionProducer, GeneticAlgorithmSolutionScorer, MultiInputSolutionScorer, NGramSolutionScorer, SimpleGeneticAlgorithmRunner, GeneticAlgorithmRunner, DictionarySolutionScorer, CombinedSolutionScorer, ScoringFactor, NonAlphabeticSolutionScorer};

#[derive(Debug)]
struct WeakSaltSolution {
    key: Vec<u8>,
}

struct WeakSaltSolutionProducer {
    key_length: usize,
}

impl GeneticAlgorithmSolution for WeakSaltSolution {
    type OriginalType = String;
    type EncodedType = Vec<u8>;

    fn decode(&self, encoded_data: &Vec<u8>) -> String {
        encoded_data.iter().zip(self.key.iter().cycle())
            .map(|(data_byte, key_byte)| data_byte ^ key_byte)
            .map(|v| v as char)
            .collect()
    }

    fn randomly_change(&self) -> Self {
        let mut key = self.key.clone();
        let change_index = (0..key.len()).choose(&mut rand::thread_rng()).unwrap();
        key[change_index] = rand::thread_rng().gen();

        Self {
            key,
        }
    }

    fn randomly_combine(&self, other: &Self) -> Self {
        let mut key = self.key.clone();
        for i in 0..key.len() {
            if rand::thread_rng().gen_bool(0.5) {
                key[i] = other.key[i];
            }
        }

        Self {
            key
        }
    }
}

impl GeneticAlgorithmSolutionProducer<WeakSaltSolution> for WeakSaltSolutionProducer {
    fn random(&self) -> WeakSaltSolution {
        WeakSaltSolution {
            key: (&mut rand::thread_rng()).sample_iter(Uniform::new(u8::MIN, u8::MAX)).take(self.key_length).collect(),
        }
    }
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Weak Salt");

    let input = fs::read_to_string("input").unwrap();
    let lines: Vec<Vec<u8>> = input.lines()
        .map(|v| hex::decode(v).unwrap())
        .collect();
    let max_key_length = lines.iter().map(|v| v.len()).max().unwrap();

    let producer = WeakSaltSolutionProducer {
        key_length: max_key_length,
    };

    info!("Setting up scorer");
    let scorer = MultiInputSolutionScorer::new(
        CombinedSolutionScorer::new(vec![
            Box::new(ScoringFactor::new(NGramSolutionScorer::new(1), 0.1)),
            Box::new(ScoringFactor::new(NGramSolutionScorer::new(2), 0.5)),
            Box::new(NGramSolutionScorer::new(3)),
            //Box::new(NGramSolutionScorer::new(5)),
            //Box::new(NGramSolutionScorer::new(7)),
            //Box::new(ScoringFactor::new(NonAlphabeticSolutionScorer::new(), -0.05)),
        ])
    );
    info!("Done setting up scorer");

    let mut runner = SimpleGeneticAlgorithmRunner::new(
        producer,
        scorer,
        lines.clone(),
    );

    let mut best_iteration_score = f32::MIN;

    loop {
        runner.run_iteration();

        let (solution, score) = runner.best_solution();
        if *score > best_iteration_score {
            info!("Best solution so far: {:?} {}", solution, score);
            for line in &lines {
                println!("{}", solution.decode(&line).chars().map(|v| if ('a'..='z').contains(&v) || v.is_ascii_punctuation() {
                    v
                } else {
                    ' '
                }).collect::<String>());
            }
            best_iteration_score = *score;
        }
    }
}