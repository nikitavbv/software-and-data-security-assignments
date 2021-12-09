use rand::prelude::*;
use rand::distributions::Alphanumeric;
use lazy_static::lazy_static;

lazy_static! {
    static ref TOP_ENGLISH_WORDS: (Vec<String>, Vec<f32>) = load_top_english_words();
}

fn main() {
    println!("Hello, world!");
}

/*fn generate_human_like_password() -> String {
    let t = rand::thread_rng().gen_range(0..100);
    if t <= 3 {
        generate_really_random_password()
    } else if t <= 8 {
        generate_human_like_password_with_words()
    } else if t <= 16 {
        generate_top_password()
    } else {
        generate_most_common_list_password()
    }
}

fn generate_really_random_password() -> String {
    (0..rand::thread_rng().gen_range(8..=16))
        .map(|_| if thread_rng().gen_bool(0.03) {
            ['!', '.', '?', '%'].choose(&mut thread_rng())
        } else {
            thread_rng().sample(Alphanumeric)
        })
        .collect()
}

fn generate_human_like_password_with_words() -> String {

}*/

fn load_top_english_words() -> (Vec<String>, Vec<f32>) {
    let mut words = Vec::new();
    let mut frequencies = Vec::new();



    (words, frequencies)
}