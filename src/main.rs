use actix_web::{post, web, App, HttpServer};

use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use std::sync::{Arc, Mutex};

use std::env;

// App state
struct AppState {
    buffer: Arc<Mutex<File>>,
}

impl AppState {
    fn new(buffer: Arc<Mutex<File>>) -> AppState {
        return AppState { buffer };
    }
}

#[post("/log")]
async fn log(data: web::Data<AppState>, req_body: String) -> String {
    println!("Received post: {}", req_body);

    let file = &data.buffer;
    let mut file = match file.lock() {
        Ok(file) => file,
        Err(error) => return format!("Unable to lock to write to file: {}", error),
    };
    let write_result = match file.write(format!("{}\n", req_body).as_bytes()) {
        Ok(_) => String::from("Ok"),
        Err(error) => format!("Unable to log to file: {}", error),
    };

    write_result
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log_file: String = env::var("LOG_FILE").unwrap_or(String::from("log.log"));

    let file = Arc::new(Mutex::new(
        OpenOptions::new()
            .append(true)
            .create(true)
            .open(log_file)?,
    ));

    HttpServer::new(move || App::new().data(AppState::new(file.clone())).service(log))
        .bind("0.0.0.0:8899")?
        .run()
        .await
}
