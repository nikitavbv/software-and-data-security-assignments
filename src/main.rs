use vigenere::{VigenereKey, guess_key_length};
use crate::{substitution::SubstitutionKey, vigenere::guess_key, wonderland::WONDERLAND};

mod substitution;
mod vigenere;
mod wonderland;

fn main() {
    /*let key = SubstitutionKey::random();
    let encoded = key.encode(&WONDERLAND);
    guess_key(&encoded);*/

    let key = VigenereKey::random();
    let encoded = key.encode(&WONDERLAND);
    println!("key: {:?}, actual length: {}", key, key.key.len());
    // let key_length = guess_key_length(&encoded);
    // println!("key length: {}", key_length);
    guess_key(&encoded, key.key.len());
}
