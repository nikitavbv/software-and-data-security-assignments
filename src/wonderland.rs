use std::collections::HashSet;
use lazy_static::lazy_static;

pub const WONDERLAND: &str = include_str!("wonderland.txt");

lazy_static! {
    pub static ref ALPHABET: Vec<char> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    ];

    pub static ref ENGLISH_DICTIONARY: HashSet<String> = load_english_words();
}

fn load_english_words() -> HashSet<String> {
    println!("loading english words");
    let mut result = HashSet::new();
    let words = include_str!("words.txt").lines();
    for word in words {
        result.insert(word.to_ascii_lowercase().replace("\n", "").trim().to_string());
    }
    println!("done loading english words");

    result
}