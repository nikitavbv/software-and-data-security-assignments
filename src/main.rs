use substitution::guess_key;
use vigenere::{VigenereKey, guess_key_length};

use crate::{substitution::SubstitutionKey, wonderland::WONDERLAND};

mod substitution;
mod vigenere;
mod wonderland;

fn main() {
    /*let key = SubstitutionKey::random();
    let encoded = key.encode(&WONDERLAND);
    guess_key(&encoded);*/

    let key = VigenereKey::random();
    let encoded = key.encode(&WONDERLAND);
    println!("key: {:?}", key);
    guess_key_length(&encoded);
}
