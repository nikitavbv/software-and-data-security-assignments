use substitution::guess_key;

use crate::{substitution::SubstitutionKey, wonderland::WONDERLAND};

mod substitution;
mod wonderland;

fn main() {
    let key = SubstitutionKey::random();
    let encoded = key.encode(&WONDERLAND);
    guess_key(&encoded);
}
