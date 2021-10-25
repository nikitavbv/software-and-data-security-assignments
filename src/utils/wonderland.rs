use std::collections::HashSet;
use std::ops::Index;
use lazy_static::lazy_static;

pub const WONDERLAND: &str = include_str!("wonderland.txt");

lazy_static! {
    pub static ref ENGLISH_ALPHABET: Vec<char> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    pub static ref RUSSIAN_ALPHABET: Vec<char> = vec![
        'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р',
        'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
    ];
    pub static ref UKRAINIAN_ALPHABET: Vec<char> = vec![
        'І',
    ];
    pub static ref NUMBERS: Vec<char> = vec![
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
    ];
    pub static ref SYMBOLS: Vec<char> = vec![
        '!', '.', ',', '-', '–', ':', '%', '«', '»', '(', ')', '/', '?', '=',
    ];
    pub static ref COMBINED_ALPHABET: Vec<char> = {
        let mut alphabet = Vec::new();
        alphabet.extend(&ENGLISH_ALPHABET.clone());
        alphabet.extend(&RUSSIAN_ALPHABET.clone());
        alphabet.extend(&UKRAINIAN_ALPHABET.clone());
        alphabet.extend(&NUMBERS.clone());
        alphabet.extend(&SYMBOLS.clone());
        alphabet
    };

    pub static ref ENGLISH_DICTIONARY: HashSet<String> = load_english_words();
    pub static ref RUSSIAN_DICTIONARY: HashSet<String> = load_russian_words();
    pub static ref UKRAINIAN_DICTIONARY: HashSet<String> = load_ukrainian_words();
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

fn load_russian_words() -> HashSet<String> {
    println!("loading russian words");
    let mut result = HashSet::new();
    let words = include_str!("words_ru.txt").lines();
    for word in words {
        result.insert(word.to_lowercase().replace("\n", "").trim().to_string());
    }
    println!("done loading russian dictionary");

    result
}

fn load_ukrainian_words() -> HashSet<String> {
    println!("loading ukrainian words");
    let mut result = HashSet::new();
    let words = include_str!("words_ua.txt").lines();
    for word in words {
        let word = word.to_lowercase();
        let word = word.replace("\n", "").to_string();
        let word = word.trim().to_string();
        let word = if word.contains("/") {
            word[0..word.find("/").unwrap()].to_string()
        } else {
            word
        };
        result.insert(word);
    }
    println!("done loading ukrainian words");

    result
}