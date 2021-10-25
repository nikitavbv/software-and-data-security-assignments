use std::fs;
use log::info;
use crate::utils::dictionaries::count_english_words;

const ENGLISH_WORDS_THRESHOLD: u32 = 10;

pub fn run() {
    let input = fs::read_to_string("tasks/task3/input").unwrap().replace("\n", "");
    //let decoded = base64::decode(&input).unwrap();
    let decoded: Vec<u8> = hex::decode(&input).unwrap();


    for key in 0..=255 {
        let mut result = Vec::new();
        for i in 0..decoded.len() {
            result.push((decoded[i] as u8 ^ key));
        }

        let result_str: String = String::from_utf8_lossy(&result).to_string();
        if count_english_words(&result_str) as u32 > ENGLISH_WORDS_THRESHOLD {
            info!("key = {}", key);
            fs::write("./tasks/task3/output", result_str).unwrap();
            return;
        }
    }
}