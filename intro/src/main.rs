use std::collections::HashMap;
use std::path::Path;
use env_logger::Env;
use hex::encode;
use indicatif::ProgressIterator;

use log::info;
use rand::prelude::{IteratorRandom, SliceRandom};

mod tasks;
mod utils;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    if !Path::new("tasks/task1/output").exists() {
        info!("Running task1");
        tasks::task1::run();
    }

    if !Path::new("tasks/task2/output").exists() {
        info!("Running task2");
        tasks::task2::run();
    }

    if !Path::new("tasks/task3/output").exists() {
        info!("Running task3");
        tasks::task3::run();
    }

    if !Path::new("tasks/task4/output").exists() {
        info!("Running task4");
        tasks::task4::run();
    }

    if !Path::new("tasks/task5/output").exists() {
        info!("Running task5");
        tasks::task5::run();
    }

    info!("Finished running tasks");
}

#[derive(Debug, Clone)]
struct Solution {
    key: Vec<Key>,
}

#[derive(Debug, Clone)]
struct Key {
    alphabet: [char; 26],
}

fn run() {
    let encoded_text = "UMUPLYRXOYRCKTYYPDYZTOUYDZHYJYUNTOMYTOLTKAOHOKZCMKAVZDYBRORPTHQLSERUOERMKZGQJOIDJUDNDZATUVOTTLMQBOWNMERQTDTUFKZCMTAZMEOJJJOXMERKJHACMTAZATIZOEPPJKIJJNOCFEPLFBUNQHHPPKYYKQAZKTOTIKZNXPGQZQAZKTOTIZYNIUISZIAELMKSJOYUYYTHNEIEOESULOXLUEYGBEUGJLHAJTGGOEOSMJHNFJALFBOHOKAGPTIHKNMKTOUUUMUQUDATUEIRBKYUQTWKJKZNLDRZBLTJJJIDJYSULJARKHKUKBISBLTOJRATIOITHYULFBITOVHRZIAXFDRNIORLZEYUUJGEBEYLNMYCZDITKUXSJEJCFEUGJJOTQEZNORPNUDPNQIAYPEDYPDYTJAIGJYUZBLTJJYYNTMSEJYFNKHOTJARNLHHRXDUPZIALZEDUYAOSBBITKKYLXKZNQEYKKZTOKHWCOLKURTXSKKAGZEPLSYHTMKRKJIIQZDTNHDYXMEIRMROGJYUMHMDNZIOTQEKURTXSKKAGZEPLSYHTMKRKJIIQZDTNROAUYLOTIMDQJYQXZDPUMYMYPYRQNYFNUYUJJEBEOMDNIYUOHYYYJHAOQDRKKZRRJEPCFNRKJUHSJOIRQYDZBKZURKDNNEOYBTKYPEJCMKOAJORKTKJLFIOQHYPNBTAVZEUOBTKKBOWSBKOSKZUOZIHQSLIJJMSURHYZJJZUKOAYKNIYKKZNHMITBTRKBOPNUYPNTTPOKKZNKKZNLKZCFNYTKKQNUYGQJKZNXYDNJYYMEZRJJJOXMERKJVOSJIOSIQAGTZYNZIOYSMOHQDTHMEDWJKIULNOTBCALFBJNTOGSJKZNEEYYKUIXLEUNLNHNMYUOMWHHOOQNUYGQJKZLZJZLOLATSEHQKTAYPYRZJYDNQDTHBTKYKYFGJRRUFEWNTHAXFAHHODUPZMXUMKXUFEOTIMUNQIHGPAACFKATIKIZBTOTIKZNKKZNLORUKMLLFBUUQKZNLEOHIEOHEDRHXOTLMIRKLEAHUYXCZYTGUYXCZYTIUYXCZYTCVJOEBKOHE";

    info!("loading ngrams...");
    let mut all_ngrams: HashMap<String, u32> = HashMap::new();
    let ngrams_sizes = [2, 3, 5];

    for i in ngrams_sizes {
        generate_ngrams(i);
        let trigrams: HashMap<String, u32> = bincode::deserialize(&std::fs::read(format!("ngrams-{}", i)).unwrap()).unwrap();

        for (k, v) in trigrams {
            all_ngrams.insert(k, v);
        }
    }
    info!("loaded");

    let mut solutions: Vec<(Solution, u32)> = Vec::new();
    let mut best_score_so_far: u32 = 0;

    for i in 0..10000 {
        let solution = improve_solution(random_solution(), &encoded_text);
        let decoded_text = decode(&encoded_text, &solution);
        let score = score_decoded_text(&all_ngrams, &ngrams_sizes, &decoded_text);
        solutions.push((solution, score));
    }

    loop {
        solutions.sort_by(|a, b| a.1.cmp(&b.1));

        let (best_solution, best_solution_score) = &solutions[solutions.len() - 1];
        if *best_solution_score > best_score_so_far {
            best_score_so_far = *best_solution_score;
            let decoded_text = decode(&encoded_text, &best_solution);
            println!("best solution so far: {} {} {:?}", best_score_so_far, decoded_text, best_solution);
        }

        solutions.drain(0..5000);

        for i in 0..4000 {
            let (random_solution, _) = solutions.choose(&mut rand::thread_rng()).unwrap();
            let solution = improve_solution(randomly_change(random_solution), &encoded_text);
            let decoded_text = decode(&encoded_text, &solution);
            let score = score_decoded_text(&all_ngrams, &ngrams_sizes, &decoded_text);
            solutions.push((solution, score));
        }

        while solutions.len() < 10000 {
            let solution = improve_solution(random_solution(), &encoded_text);
            let decoded_text = decode(&encoded_text, &solution);
            let score = score_decoded_text(&all_ngrams, &ngrams_sizes, &decoded_text);
            solutions.push((solution, score));
        }
    }
}

fn score_decoded_text(trigrams: &HashMap<String, u32>, ngrams_sizes: &[usize], text: &str) -> u32 {
    let mut score = 0;
    let mut k = 1;

    for ngrams_size in ngrams_sizes {
        for i in 0..(text.len() - ngrams_size) {
            let trigram = &text[i..i+ngrams_size];
            score += trigrams.get(trigram).unwrap_or(&0) * k;
        }

        k *= 10;
    }

    score
}

fn decode(encoded: &str, solution: &Solution) -> String {
    let encoded = encoded.to_uppercase();
    let mut decoded = Vec::new();

    for i in 0..encoded.chars().count() {
        let key = &solution.key[i % solution.key.len()];
        decoded.push(decode_char(&encoded.chars().nth(i).unwrap(), &key));
    }

    decoded.into_iter().collect()
}

fn decode_char(encoded: &char, key: &Key) -> char {
    let index = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ].iter().position(|c| c == encoded).unwrap();

    key.alphabet[index]
}

fn randomly_change(solution: &Solution) -> Solution {
    let mut key = solution.key.clone();

    //for _ in 0..3 {
    let key_to_change = (0..key.len()).choose(&mut rand::thread_rng()).unwrap();
    let mut key_to_change = key.get_mut(key_to_change).unwrap();

    let index_to_swap = (0..26).choose(&mut rand::thread_rng()).unwrap();
    let index_to_swap_with = (0..26).choose(&mut rand::thread_rng()).unwrap();

    let t = key_to_change.alphabet[index_to_swap_with];
    key_to_change.alphabet[index_to_swap_with] = key_to_change.alphabet[index_to_swap];
    key_to_change.alphabet[index_to_swap] = t;
    //}

    Solution {
        key
    }
}

fn improve_solution(mut s: Solution, encoded_text: &str) -> Solution {
    let obvious_text = "CONGRATULATIONS THIS WASNT QUITE AN EASY TASK NOW ALL THIS TEXT IS JUST GARBAGE TO LET YOU USE SOME FREQUENCY ANALYSIS".replace(" ", "");

    for i in 0..obvious_text.chars().count() {
        let key_len = s.key.len();
        let key = s.key.get_mut(i % key_len).unwrap();

        let encoded_char = encoded_text.chars().nth(i).unwrap();
        let index = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
            'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ].iter().position(|c| c == &encoded_char).unwrap();

        key.alphabet[index] = obvious_text.chars().nth(i).unwrap();
    }

    // TODO: fix remaining letters

    s
}

fn random_solution() -> Solution {
    // let total_keys = (1..=20).choose(&mut rand::thread_rng()).unwrap();
    let total_keys = 4;
    Solution {
        key: (0..total_keys).map(|_| random_key()).collect(),
    }
}

fn random_key() -> Key {
    let mut key = Key {
        alphabet: [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
            'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ]
    };
    key.alphabet.shuffle(&mut rand::thread_rng());

    key
}

fn generate_ngrams(ngram_size: usize) {
    let file_name = format!("ngrams-{}", ngram_size);

    if Path::new(&file_name).exists() {
        return;
    }

    info!("loading");
    let shakespeare = std::fs::read_to_string("../data/shakespeare.txt").unwrap().to_uppercase();
    info!("filtering");
    let filtered: String = shakespeare.chars().filter(|c| c.is_alphabetic()).collect();
    info!("counting");

    let mut trigrams: HashMap<String, u32> = HashMap::new();
    for i in (0..filtered.len() - ngram_size).progress() {
        let trigram = &filtered[i..i+ngram_size];
        trigrams.insert(trigram.to_string(), trigrams.get(trigram).unwrap_or(&0) + 1);
    }

    info!("saving");
    std::fs::write(file_name, bincode::serialize(&trigrams).unwrap()).unwrap();
}