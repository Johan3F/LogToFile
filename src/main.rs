use actix_web::{post, web, App, HttpServer};

use std::fs::OpenOptions;
use std::io::prelude::*;

use std::env;

// This struct represents state
struct AppState {
    file_path: String,
}

impl AppState {
    fn new(file_path: &str) -> AppState {
        return AppState {
            file_path: String::from(file_path),
        };
    }
}

#[post("/log")]
async fn log(data: web::Data<AppState>, req_body: String) -> String {
    println!("Receive post: {}", req_body);
    let file_path = &data.file_path;

    let file = OpenOptions::new().append(true).create(true).open(file_path);
    if file.is_err() {
        return String::from("Unable to log: Error when opening the file");
    }
    let mut file = file.unwrap();

    let result = file.write(format!("{}\n", req_body).as_bytes());
    if result.is_err() {
        return String::from("Unable to log: Error when writing to file");
    }
    String::from("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log_file: String = env::var("LOG_FILE").unwrap_or(String::from("log.log"));

    HttpServer::new(move || App::new().data(AppState::new(&log_file)).service(log))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}
