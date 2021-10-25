use std::fs;

pub fn run() {
    let text = fs::read_to_string("./input").unwrap();
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
