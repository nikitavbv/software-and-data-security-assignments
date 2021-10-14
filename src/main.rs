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
    lab1();

    // task2();
    // assumption1();
    //let encoded_link = "";
    //let decoded_link = "https://docs.google.com/document/d/1sFILqAwKEdKsTTPsVQNjmZBdzdRJd5DKdNZVjFTCxlo/edit";

    //solution();
}

fn lab1() {
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