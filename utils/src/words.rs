use std::fs;
use std::path::Path;
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
        .map(|v| v.to_string())
        .collect();
    let total_words = words.len();

    let mut words_stats: HashMap<String, f32> = HashMap::new();
    for word in words {
        let word = word.replace("\n", "").replace(" ", "");
        words_stats.insert(word.clone(), words_stats.get(&word).unwrap_or(&0.0) + 1.0);
    }

    let mut result = HashMap::new();
    for (k, v) in words_stats.iter() {
        result.insert(k.clone(), v / total_words as f32);
    }

    fs::write(file_path, bincode::serialize(&result).unwrap()).unwrap();
    result
}