use crate::{substitution::SubstitutionKey, wonderland::WONDERLAND};

mod substitution;
mod wonderland;

fn main() {
    let key = SubstitutionKey::random();

    println!("Hello, world! {}", key.encode(WONDERLAND));
}
