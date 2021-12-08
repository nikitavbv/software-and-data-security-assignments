use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize)]
struct PlayResponse {
    message: String,
    account: AccountState,
    #[serde(rename="realNumber")]
    real_number: i32,
}

#[derive(Deserialize)]
struct AccountState {
    money: u32,
}

fn main() {
    println!("Hello, world!");

    // 800739223
    // -1436932086
    // -1208944671
    // -882364468
    // -1095718725

    for a in 0..i32::MAX {
        for c in 0..i32::MAX {
            // TODO: try to crack by brute-force
        }
    }

    /*let player_id: u32 = rand::thread_rng().gen();
    create_account(player_id);

    for i in 0..10 {
        let res = make_bet(player_id, "Lcg".to_owned(), 1, 0);
        println!("res is: {}", res.real_number);
    }*/
}

fn lcg_next(last: i32, a: i32, c: i32) -> i32 {
    let m = 2 ** 32;
    (a * last + c) % m
}

fn create_account(player_id: u32) {
    reqwest::blocking::get(format!("http://95.217.177.249/casino/createacc?id={}", player_id)).unwrap();
}

fn make_bet(player_id: u32, mode: String, amount_of_money: u32, number: u32) -> PlayResponse {
    reqwest::blocking::get(format!("http://95.217.177.249/casino/play{}?id={}&bet={}&number={}", mode, player_id, amount_of_money, number))
        .unwrap()
        .json()
        .unwrap()
}