use std::fs;
use std::fs::File;
use std::io::{LineWriter, Write};
use std::path::Path;
use argon2::Config;
use indicatif::ProgressIterator;
use rand::prelude::*;
use rand::distributions::{Alphanumeric, WeightedIndex};
use lazy_static::lazy_static;
use rand::rngs::OsRng;
use sha1::{Sha1, Digest};
use utils::words::english_words;

lazy_static! {
    static ref TOP_ENGLISH_WORDS: (Vec<String>, Vec<f32>) = load_top_english_words();
    static ref TOP_PASSWORDS: Vec<String> = load_top_passwords();
    static ref COMMON_PASSWORDS: Vec<String> = load_common_passwords();
}

fn main() {
    println!("Human-like password generator");

    if !Path::new("output/weak_hashes").exists() {
        generate_weak_hashes();
    }

    if !Path::new("output/strong_hashes").exists() {
        generate_strong_hashes();
    }
}

fn generate_weak_hashes() {
    println!("Generating weak hashes");
    let file = File::create("output/weak_hashes").unwrap();
    let mut file = LineWriter::new(file);

    for i in (0..1_000_000).progress() {
        let mut password = generate_human_like_password().as_bytes().to_vec();
        let salt = generate_salt();
        password.append(&mut salt.clone());

        let mut hasher = Sha1::new();
        hasher.update(&password);
        let result = hasher.digest().bytes();

        file.write(format!("{};{}\n", hex::encode(&result), base64::encode(&salt)).as_bytes()).unwrap();
    }

    file.flush();
}

fn generate_strong_hashes() {
    println!("Generating strong hashes");
    let file = File::create("output/strong_hashes").unwrap();
    let mut file = LineWriter::new(file);

    for i in (0..1_000_000).progress() {
        let mut password = generate_human_like_password().as_bytes().to_vec();
        let salt = generate_salt();
        password.append(&mut salt.clone());

        let result = argon2::hash_encoded(&password, &salt, &Config::default()).unwrap();

        file.write(format!("{}\n", result).as_bytes()).unwrap();
    }

    file.flush();
}

fn generate_salt() -> Vec<u8> {
    (0..64).map(|_| thread_rng().gen::<u8>()).collect()
}

fn generate_human_like_password() -> String {
    let t = rand::thread_rng().gen_range(0..100);
    if t <= 3 {
        generate_really_random_password()
    } else if t <= 8 {
        generate_human_like_password_with_words()
    } else if t <= 16 {
        generate_top_password()
    } else {
        generate_common_list_password()
    }
}

fn generate_really_random_password() -> String {
    (0u32..rand::thread_rng().gen_range(8..=16))
        .map(|_| if thread_rng().gen_bool(0.03) {
            *(['!', '.', '?', '%'].choose(&mut thread_rng()).unwrap())
        } else {
            thread_rng().sample(Alphanumeric) as char
        })
        .collect()
}

fn generate_human_like_password_with_words() -> String {
    let mut password = "".to_string();

    let target_len = rand::thread_rng().gen_range(8..=16);
    while password.len() < target_len {
        password = format!("{}{}", password, if rand::thread_rng().gen_bool(0.2) {
            rand::thread_rng().gen_range(0..10000).to_string()
        } else {
            generate_random_word()
        });
    }

    password
}

fn generate_top_password() -> String {
    TOP_PASSWORDS.choose(&mut thread_rng()).unwrap().to_string()
}

fn generate_common_list_password() -> String {
    COMMON_PASSWORDS.choose(&mut thread_rng()).unwrap().to_string()
}

fn generate_random_word() -> String {
    let dist = WeightedIndex::new(&TOP_ENGLISH_WORDS.1).unwrap();
    TOP_ENGLISH_WORDS.0[dist.sample(&mut rand::thread_rng())].clone().replace("\n", "").replace(" ", "")
}

fn load_top_passwords() -> Vec<String> {
    fs::read_to_string("top-passwords.txt").unwrap().lines().map(|v| v.to_string().replace("\n", "")).collect()
}

fn load_common_passwords() -> Vec<String> {
    fs::read_to_string("top-1kk-passwords.txt").unwrap().lines().map(|v| v.to_string().replace("\n", "")).collect()
}

fn load_top_english_words() -> (Vec<String>, Vec<f32>) {
    let mut words = Vec::new();
    let mut frequencies = Vec::new();

    for (word, frequency) in english_words() {
        words.push(word);
        frequencies.push(frequency);
    }

    (words, frequencies)
}