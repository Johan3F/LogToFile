use actix_web::{post, web, App, HttpServer};

use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use std::sync::{Arc, Mutex};

use std::env;

// App state
struct AppState {
    buffer: Arc<Mutex<dyn Write>>,
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

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{test, App};
    use std::str;

    fn new(buffer: Arc<Mutex<Vec<u8>>>) -> AppState {
        return AppState { buffer };
    }

    #[actix_rt::test]
    async fn test_index_get() {
        let expected_string = String::from("test-string");
        let buffer = Arc::new(Mutex::new(Vec::new()));

        let mut app = test::init_service(App::new().data(new(buffer.clone())).service(log)).await;

        let req = test::TestRequest::post()
            .uri("/log")
            .set_payload(expected_string.clone())
            .to_request();

        let resp = test::read_response(&mut app, req).await;

        assert_eq!(resp, String::from("Ok"));

        let gotten_buffer = &*buffer.lock().unwrap();
        let gotten_string = match str::from_utf8(gotten_buffer) {
            Ok(string) => string,
            Err(error) => panic!(error),
        };
        let expected_string = format!("{}\n", expected_string);
        assert_eq!(gotten_string, expected_string);
    }
}
