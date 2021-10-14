use std::fs;
use std::fs::File;
use std::io::prelude::*;
use vigenere::{VigenereKey, guess_key_length};
use crate::{substitution::SubstitutionKey, wonderland::WONDERLAND};
use crate::substitution::guess_key;

mod substitution;
mod vigenere;
mod wonderland;

fn main() {
    task2()
}

fn task2() {
    let text = fs::read_to_string("./tasks/task2.txt").unwrap();
    let mut bits = Vec::new();

    for char in text.chars() {
        bits.push(if char == '1' {
            1
        } else if char == '0' {
            0
        } else {
            continue;
        });
    }

    let mut bytes = Vec::new();
    while bits.len() > 0 {
        let mut byte = 0;
        for _ in 0..8 {
            byte = (byte << 1) | bits.remove(0);
        }
        bytes.push(byte as u8);
    }

    let res = base64::decode(&bytes).unwrap();
    println!("result:\n{}", String::from_utf8_lossy(&res));
}

fn task1() {
    let text = fs::read_to_string("./task1.txt").unwrap();
    let mut result = Vec::new();
    let mut index = 0;

    while index < text.chars().count() {
        result.push(text.chars().nth(index + 3).unwrap());
        result.push(text.chars().nth(index + 0).unwrap());
        result.push(text.chars().nth(index + 2).unwrap());
        result.push(text.chars().nth(index + 1).unwrap());
        index = index + 4;
    }

    println!("result is: {}", result.iter().collect::<String>().replace("!", " "));
}
