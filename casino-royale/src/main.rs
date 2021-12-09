#![allow(arithmetic_overflow)]

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

    let player_id: u32 = rand::thread_rng().gen();
    create_account(player_id);
    lcg_to_the_moon(player_id);
}

fn lcg_to_the_moon(player_id: u32) {
    println!("Cracking lcg");

    let s0 = make_bet(player_id, "Lcg".to_owned(), 1, 0).real_number;
    let s1 = make_bet(player_id, "Lcg".to_owned(), 1, 0).real_number;
    let s2 = make_bet(player_id, "Lcg".to_owned(), 1, 0).real_number;
    let (a, c) = crack_lcg(s0, s1, s2);
    println!("a is {}, c is {}", a, c);

    let mut money = 100;
    let mut last = s2;
    while money < 1_000_000 {
        let res = make_bet(player_id, "Lcg".to_owned(), money, lcg_next(last, a, c));
        last = res.real_number;
        money = res.account.money;
        println!("res is: {}, money is {}", res.message, money);
    }
}

fn crack_lcg(s0: i32, s1: i32, s2: i32) -> (i32, i32) {
    let s0t = (s0 as u32) as i64;
    let s1t = (s1 as u32) as i64;
    let s2t = (s2 as u32) as i64;

    let a = (s2t - s1t) * (modinv(s1t - s0t, 2_i64.pow(32)) as u32 as i64) % 2_i64.pow(32);
    let c = (s1t - s0t * a) as u32 as i64;

    (a as i32, c as i32)
}

fn lcg_next(last: i32, a: i32, c: i32) -> i32 {
    a * last + c
}

// Extended euclidian algorithm
fn modinv(b: i64, n: i64) -> i64 {
    let (g, x, _) = egcd(b, n);
    if g == 1 {
        x % n
    } else {
        panic!("g != 1");
    }
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn create_account(player_id: u32) {
    reqwest::blocking::get(format!("http://95.217.177.249/casino/createacc?id={}", player_id)).unwrap();
}

fn make_bet(player_id: u32, mode: String, amount_of_money: u32, number: i32) -> PlayResponse {
    reqwest::blocking::get(format!("http://95.217.177.249/casino/play{}?id={}&bet={}&number={}", mode, player_id, amount_of_money, number))
        .unwrap()
        .json()
        .unwrap()
}