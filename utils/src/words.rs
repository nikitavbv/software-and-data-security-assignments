use std::fs;
use std::collections::HashMap;

pub fn english_words() -> HashMap<String, f32> {
    let file_path = "../data/words";
    if Path::new(file_path).exists() {
        return bincode::deserialize(&fs::read(file_path).unwrap()).unwrap();
    }

    let shakespeare = fs::read_to_string("../data/shakespeare.txt").unwrap();
    let words: Vec<String> = shakespeare
        .chars()
        .filter(|v| v.is_alphabetic() || v.is_whitespace())
        .collect::<String>()
        .split(" ")
        .filter(|v| !v.replace(" ", "").is_empty())
        .collect();
    let total_words = words.len();

    let mut words_stats = HashMap::new();
    for word in words {
        // TODO: count words
    }

    fs::write(file_path, bincode::serialize(&words_stats).unwrap()).unwrap();
    words_stats
}