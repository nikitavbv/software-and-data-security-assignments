#![allow(arithmetic_overflow)]

use std::time::UNIX_EPOCH;
use chrono::Utc;
use mersenne_twister::{MersenneTwister, MT19937};
use rand::{Rng, SeedableRng};
use serde::Deserialize;

#[derive(Deserialize)]
struct PlayResponse {
    message: String,
    account: AccountState,
    #[serde(rename="realNumber")]
    real_number: i64,
}

#[derive(Deserialize)]
struct AccountState {
    money: u32,
}

fn main() {
    println!("Hello, world!");

    let player_id: u32 = rand::thread_rng().gen();
    create_account(player_id);

    // lcg_to_the_moon(player_id);
    mersenne_twister_to_the_moon(player_id);
}

fn mersenne_twister_to_the_moon(player_id: u32) {
    println!("Cracking Mersenne Twister 19937 while knowning the seed");

    let s0 = make_bet_mt(player_id, 1, 0).real_number;
    println!("s0 is {}", s0);

    let timestamp = Utc::now().timestamp() as i32;
    let mut rng = None;
    for i in -10..10 {
        let mut rng_attempt: MT19937 = MT19937::from_seed((timestamp+ i) as u32);
        let generated_number = rng_attempt.gen::<u32>();
        if generated_number == s0 as u32 {
            rng = Some(rng_attempt);
            break;
        }
    }

    let mut rng = rng.unwrap();
    let mut money = 100;
    while money < 1_000_000 {
        let next = rng.gen::<u32>();
        println!("next is {}", next);
        let res = make_bet_mt(player_id, money, next);
        money = res.account.money;
        println!("res is: {}, money is {}", res.message, money);
    }
}

fn lcg_to_the_moon(player_id: u32) {
    println!("Cracking lcg");

    let s0 = make_bet_lcg(player_id, 1, 0).real_number;
    let s1 = make_bet_lcg(player_id, 1, 0).real_number;
    let s2 = make_bet_lcg(player_id, 1, 0).real_number;
    let (a, c) = crack_lcg(s0, s1, s2);
    println!("a is {}, c is {}", a, c);

    let mut money = 100;
    let mut last = s2;
    while money < 1_000_000 {
        let res = make_bet_lcg(player_id, money, lcg_next(last as i32, a, c));
        last = res.real_number;
        money = res.account.money;
        println!("res is: {}, money is {}", res.message, money);
    }
}

fn crack_lcg(s0: i64, s1: i64, s2: i64) -> (i32, i32) {
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

fn make_bet_mt(player_id: u32, amount_of_money: u32, number: u32) -> PlayResponse {
    let res = reqwest::blocking::get(format!("http://95.217.177.249/casino/playMt?id={}&bet={}&number={}", player_id, amount_of_money, number))
        .unwrap()
        .text()
        .unwrap();

    match serde_json::from_str(&res) {
        Ok(v) => v,
        Err(err) => panic!("Failed to deserialize response: {} {:?}", &res, err),
    }
}

fn make_bet_lcg(player_id: u32, amount_of_money: u32, number: i32) -> PlayResponse {
    let res = reqwest::blocking::get(format!("http://95.217.177.249/casino/playLcg?id={}&bet={}&number={}", player_id, amount_of_money, number))
        .unwrap()
        .text()
        .unwrap();

    match serde_json::from_str(&res) {
        Ok(v) => v,
        Err(err) => panic!("Failed to deserialize response: {} {:?}", &res, err),
    }
}