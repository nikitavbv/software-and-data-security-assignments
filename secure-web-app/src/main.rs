use std::env;
use actix_web::{HttpResponse, HttpServer};
use actix_web::web::Json;
use actix_web::{App, post, Responder};
use argon2::Config;
use env_logger::Env;
use postgres::{Client, NoTls};
use serde::Deserialize;
use rand::prelude::*;
use rand::thread_rng;
use log::info;
use jwt_simple::prelude::*;

#[derive(Deserialize)]
struct SignUpRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("starting secure-web-app");

    HttpServer::new(move || App::new()
        .service(signup)
        .service(login)
    ).bind("0.0.0.0:8080")?
        .run()
        .await
}

#[post("/api/v1/auth/signup")]
async fn signup(req: Json<SignUpRequest>) -> impl Responder {
    let mut client = Client::connect(&env::var("DATABASE_CONNECTION_STRING").unwrap(), NoTls).unwrap();

    let password = req.password.as_bytes().to_vec();
    let salt = generate_salt();

    let hashed_password = argon2::hash_encoded(&password, &salt, &Config::default()).unwrap();

    if client.query("select * from users where username = $1", &[&req.username]).unwrap().len() > 0 {
        return HttpResponse::BadRequest().body("username_already_taken");
    }

    client.execute("insert into users (username, password) values ($1, $2)", &[&req.username, &hashed_password]).unwrap();

    info!("New user signed up: {}", req.username);

    HttpResponse::Ok().body(issue_jwt(&req.username))
}

#[post("/api/v1/auth/login")]
async fn login(req: Json<LoginRequest>) -> impl Responder {
    let mut client = Client::connect(&env::var("DATABASE_CONNECTION_STRING").unwrap(), NoTls).unwrap();

    let row = client.query_one("select password from users where username = $1", &[&req.username]).unwrap();
    if row.len() == 0 {
        info!("User login failed, not found: {}", req.username);
        return HttpResponse::NotFound().body("not_found");
    }

    let password_hash: String = row.get(0);
    if !argon2::verify_encoded(&password_hash, req.password.as_bytes()).unwrap() {
        info!("User login failed, wrong password: {}", req.username);
        return HttpResponse::Forbidden().body("wrong_password");
    }

    info!("User login successful: {}", req.username);

    HttpResponse::Ok().body(issue_jwt(&req.username))
}

fn issue_jwt(username: &str) -> String {
    let key = HS384Key::from_bytes(&base64::decode(&env::var("JWT_KEY").unwrap().as_bytes()).unwrap());

    let claims = Claims::create(Duration::from_hours(1))
        .with_audience(username);

    key.authenticate(claims).unwrap()
}

fn generate_salt() -> Vec<u8> {
    (0..64).map(|_| thread_rng().gen::<u8>()).collect()
}