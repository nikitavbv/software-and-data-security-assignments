use std::fs;

use log::debug;

pub fn run() {
    let input = fs::read_to_string("tasks/task4/input").unwrap().replace("\n", "");
    let input = base64::decode(&input).unwrap();

    let key_length = guess_key_length(&input);
    info!("guessed key length as: {}", key_length);
}

pub fn guess_key_length(encoded_data: &[u8]) -> usize {
    let mut best_score = 0;
    let mut best_key_length = 0;

    for key_length in 1..100 {
        let mut coincidences = 0;
        for index in 0..encoded_data.len() {
            let index_with_offset = (index + key_length) % encoded_data.len();
            let index_char = encoded_data[index];
            let offset_char = encoded_data[index_with_offset];

            if index_char.to_ascii_lowercase() == offset_char.to_ascii_lowercase() {
                coincidences += 1;
            }
        }

        if coincidences > best_score {
            best_score = coincidences;
            best_key_length = key_length;
        }

        debug!("key length: {}, coincidences: {}", key_length, coincidences);
    }

    best_key_length
}
