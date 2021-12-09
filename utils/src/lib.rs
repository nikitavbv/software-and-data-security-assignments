use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use rand::prelude::*;
use log::info;
use indicatif::ProgressIterator;

pub mod dictionaries;
pub mod words;

pub trait GeneticAlgorithmSolution {
    type OriginalType;
    type EncodedType;
    fn decode(&self, encoded_data: &Self::EncodedType) -> Self::OriginalType;
    fn randomly_change(&self) -> Self;
    fn randomly_combine(&self, other: &Self) -> Self;
}

pub trait GeneticAlgorithmSolutionProducer<T: GeneticAlgorithmSolution> {
    fn random(&self) -> T;
}

pub trait GeneticAlgorithmSolutionScorer<T: GeneticAlgorithmSolution> {
    type InputType;
    fn score(&self, solution: &T, input_data: &Self::InputType) -> f32;
}

pub trait GeneticAlgorithmRunner<S: GeneticAlgorithmSolution, SolutionProducer: GeneticAlgorithmSolutionProducer<S>, SolutionScorer: GeneticAlgorithmSolutionScorer<S, InputType=InputType>, InputType> {
    fn new(solution_producer: SolutionProducer, solution_scorer: SolutionScorer, input_data: InputType) -> Self;

    fn run_iteration(&mut self);

    fn best_solution(&self) -> &(S, f32);
}

pub struct SimpleGeneticAlgorithmRunner<
    Solution: GeneticAlgorithmSolution,
    SolutionProducer: GeneticAlgorithmSolutionProducer<Solution>,
    SolutionScorer: GeneticAlgorithmSolutionScorer<Solution, InputType = InputType>,
    InputType
> {
    solutions: Vec<(Solution, f32)>,
    solution_producer: SolutionProducer,
    solution_scorer: SolutionScorer,
    input_data: InputType,
}

impl <S: GeneticAlgorithmSolution, SolutionProducer: GeneticAlgorithmSolutionProducer<S>, SolutionScorer: GeneticAlgorithmSolutionScorer<S, InputType = InputType>, InputType> GeneticAlgorithmRunner<S, SolutionProducer, SolutionScorer, InputType> for SimpleGeneticAlgorithmRunner<S, SolutionProducer, SolutionScorer, InputType> {
    fn new(solution_producer: SolutionProducer, solution_scorer: SolutionScorer, input_data: InputType) -> Self {
        Self {
            solutions: Vec::new(),
            solution_producer,
            solution_scorer,
            input_data,
        }
    }

    fn run_iteration(&mut self) {
        if self.solutions.len() > 5000 {
            self.solutions.drain(0..5000);

            for _ in 0..1000 {
                let (random_solution, _) = self.solutions.choose(&mut rand::thread_rng()).unwrap();
                let (another_random_solution, _) = self.solutions.choose(&mut rand::thread_rng()).unwrap();
                let new_solution = random_solution.randomly_combine(&another_random_solution);
                let score = self.solution_scorer.score(&new_solution, &self.input_data);
                self.solutions.push((new_solution, score));
            }

            for _ in 0..3000 {
                let (random_solution, _) = self.solutions.choose(&mut rand::thread_rng()).unwrap();
                let new_solution = random_solution.randomly_change();
                let score = self.solution_scorer.score(&new_solution, &self.input_data);
                self.solutions.push((new_solution, score));
            }
        }

        while self.solutions.len() < 10000 {
            self.solutions.push(self.generate_and_score());
        }

        self.solutions.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
    }

    fn best_solution(&self) -> &(S, f32) {
        self.solutions.get(self.solutions.len() - 1).unwrap()
    }
}

impl <
    Solution: GeneticAlgorithmSolution,
    SolutionProducer: GeneticAlgorithmSolutionProducer<Solution>,
    SolutionScorer: GeneticAlgorithmSolutionScorer<Solution, InputType = InputType>,
    InputType
> SimpleGeneticAlgorithmRunner<Solution, SolutionProducer, SolutionScorer, InputType> {

    fn generate_and_score(&self) -> (Solution, f32) {
        let solution = self.solution_producer.random();
        let score = self.solution_scorer.score(&solution, &self.input_data);
        (solution, score)
    }
}

pub struct NGramSolutionScorer {
    n: usize,
    ngrams: HashMap<String, f32>,
}

impl NGramSolutionScorer {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            ngrams: Self::generate_ngrams(n),
        }
    }

    fn generate_ngrams(n: usize) -> HashMap<String, f32> {
        let file_name = format!("../data/ngrams-{}", n);
        if Path::new(&file_name).exists() {
            return bincode::deserialize(&fs::read(file_name).unwrap()).unwrap();
        }

        info!("Generating ngrams of size {}", n);
        let shakespeare = fs::read_to_string("../data/shakespeare.txt").unwrap().to_uppercase();
        let filtered: String = shakespeare.chars().filter(|c| c.is_alphabetic()).collect();

        let mut ngrams: HashMap<String, f32> = HashMap::new();
        let mut total = 0.0;
        for i in (0..filtered.len() - n).progress() {
            let ngram = &filtered[i..i+n];
            ngrams.insert(ngram.to_string(), ngrams.get(ngram).unwrap_or(&0.0) + 1.0);
            total += 1.0;
        }

        for (k, v) in &ngrams.clone() {
            ngrams.insert(k.clone(), v / total);
        }

        fs::write(file_name, bincode::serialize(&ngrams).unwrap()).unwrap();

        ngrams
    }
}

impl <S: GeneticAlgorithmSolution<OriginalType = String>> GeneticAlgorithmSolutionScorer<S> for NGramSolutionScorer {
    type InputType = S::EncodedType;

    fn score(&self, solution: &S, input_data: &Self::InputType) -> f32 {
        let decoded: String = solution.decode(input_data).chars()
            .map(|v| if ('a'..='z').contains(&v) || v.is_ascii_punctuation() {
                v
            } else {
                ' '
            })
            .collect();
        if decoded.len() < self.n {
            return 0.0;
        }

        (0..decoded.len() - self.n)
            .map(|i| &decoded[i..i+self.n])
            .map(|ngram| self.ngrams.get(&ngram.to_owned().to_uppercase()).unwrap_or(&0.0))
            .sum()
    }
}

pub struct DictionarySolutionScorer {
    dictionary: HashSet<String>,
}

impl DictionarySolutionScorer {
    pub fn new() -> Self {
        Self {
            dictionary: Self::generate_dictionary(),
        }
    }

    fn generate_dictionary() -> HashSet<String> {
        let file_name = format!("../data/dictionary");
        if Path::new(&file_name).exists() {
            return bincode::deserialize(&fs::read(file_name).unwrap()).unwrap();
        }

        info!("Generating dictionary");
        let shakespeare = fs::read_to_string("../data/shakespeare.txt").unwrap().to_uppercase();
        let filtered: String = shakespeare.chars().filter(|c| c.is_alphabetic() && c.is_whitespace()).collect();

        let dictionary: HashSet<String> = filtered.split(" ")
            .filter(|entry| !entry.replace(" ", "").is_empty())
            .map(|entry| entry.to_lowercase())
            .collect();

        fs::write(file_name, bincode::serialize(&dictionary).unwrap()).unwrap();

        dictionary
    }
}

impl <S: GeneticAlgorithmSolution<OriginalType = String>> GeneticAlgorithmSolutionScorer<S> for DictionarySolutionScorer {
    type InputType = S::EncodedType;

    fn score(&self, solution: &S, input_data: &Self::InputType) -> f32 {
        solution.decode(input_data).chars()
            .filter(|v| v.is_alphabetic() || v.is_whitespace())
            .collect::<String>()
            .split(" ")
            .filter(|word| !word.replace(" ", "").is_empty())
            .map(|word| word.to_lowercase())
            .filter(|word| self.dictionary.contains(word))
            .count() as f32
    }
}

pub struct MultiInputSolutionScorer<InnerScorer> {
    inner: InnerScorer,
}

impl <InnerScorer> MultiInputSolutionScorer<InnerScorer> {
    pub fn new(inner: InnerScorer) -> Self {
        Self {
            inner,
        }
    }
}

impl <S: GeneticAlgorithmSolution, InnerScorer: GeneticAlgorithmSolutionScorer<S, InputType=S::EncodedType>> GeneticAlgorithmSolutionScorer<S> for MultiInputSolutionScorer<InnerScorer> {
    type InputType = Vec<S::EncodedType>;

    fn score(&self, solution: &S, input_data: &Self::InputType) -> f32 {
        input_data.iter().map(|entry| self.inner.score(solution, entry)).sum()
    }
}

pub struct CombinedSolutionScorer<S: GeneticAlgorithmSolution> {
    inner: Vec<Box<dyn GeneticAlgorithmSolutionScorer<S, InputType = S::EncodedType>>>,
}

impl <S: GeneticAlgorithmSolution> CombinedSolutionScorer<S> {
    pub fn new(inner: Vec<Box<dyn GeneticAlgorithmSolutionScorer<S, InputType = S::EncodedType>>>) -> Self {
        Self {
            inner,
        }
    }
}

impl <S: GeneticAlgorithmSolution> GeneticAlgorithmSolutionScorer<S> for CombinedSolutionScorer<S> {
    type InputType = S::EncodedType;
    fn score(&self, solution: &S, input_data: &S::EncodedType) -> f32 {
        self.inner.iter().map(|scorer| scorer.score(solution, input_data)).sum()
    }
}

pub struct ScoringFactor<InnerScorer> {
    factor: f32,
    inner: InnerScorer,
}

impl <InnerScorer> ScoringFactor<InnerScorer> {
    pub fn new(inner: InnerScorer, factor: f32) -> Self {
        Self {
            inner,
            factor,
        }
    }
}

impl <S: GeneticAlgorithmSolution, InnerScorer: GeneticAlgorithmSolutionScorer<S, InputType = S::EncodedType>> GeneticAlgorithmSolutionScorer<S> for ScoringFactor<InnerScorer> {
    type InputType = S::EncodedType;

    fn score(&self, solution: &S, input_data: &Self::InputType) -> f32 {
        self.factor * self.inner.score(solution, input_data)
    }
}

pub struct NonAlphabeticSolutionScorer {
}

impl NonAlphabeticSolutionScorer {

    pub fn new() -> Self {
        Self {
        }
    }
}

impl <S: GeneticAlgorithmSolution<OriginalType = String>> GeneticAlgorithmSolutionScorer<S> for NonAlphabeticSolutionScorer {
    type InputType = S::EncodedType;

    fn score(&self, solution: &S, input_data: &Self::InputType) -> f32 {
        solution.decode(input_data).chars()
            .filter(|v| !(v.is_ascii_alphabetic() || v.is_ascii_whitespace() || v.is_ascii_punctuation()))
            .count() as f32
    }
}