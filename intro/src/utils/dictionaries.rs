use std::collections::HashSet;
use std::ops::Index;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ENGLISH_ALPHABET: Vec<char> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    pub static ref NUMBERS: Vec<char> = vec![
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
    ];
    pub static ref SYMBOLS: Vec<char> = vec![
        '!', '.', ',', '-', '–', ':', '%', '«', '»', '(', ')', '/', '?', '=',
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

pub fn count_english_words(sentence: &str) -> usize {
    sentence.split(" ").filter(|word| is_english_word(word)).count()
}

fn is_english_word(word: &str) -> bool {
    ENGLISH_DICTIONARY.contains(&word.to_ascii_lowercase())
}
