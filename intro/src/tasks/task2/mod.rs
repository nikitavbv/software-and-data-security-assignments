use std::fs;

pub fn run() {
    let text = fs::read_to_string("tasks/task2/input").unwrap();
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
    fs::write("tasks/task2/output", &res);
}

// TODO: move this somewhere
fn task2_1() {
    let text = fs::read_to_string("./tasks/task2_2.txt").unwrap();

    let mut new_text = Vec::new();
    let mut index = 0;
    while index < text.len() {
        let hex = u8::from_str_radix(&text[index..index+2], 16).unwrap();
        index += 2;

        new_text.push(hex as char);
    }

    let text = new_text;

    for offset in 0..=255 {
        let mut new_text = Vec::new();

        for char in &text {
            let t = (*char as u8 ^ offset) as u8 as char;
            new_text.push(t);
        }

        let new_text: String = new_text.iter().collect();
        println!("new text is: {:?}\n\n", new_text);
    }
}
