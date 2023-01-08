use reqwest::{Client, StatusCode};

use secret_santa_client::{models::*, app::*, requests::*};

#[tokio::main]
async  fn main() {
    let mut app = App::new(Request::new(Client::new(), "http://localhost:8000".to_string()));
    app.start().await;
}
