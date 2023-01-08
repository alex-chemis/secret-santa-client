use reqwest::Client;
use dotenvy::dotenv;
use std::env;

use secret_santa_client::{app::*, requests::*};

#[tokio::main]
async  fn main() {
    dotenv().ok();

    let service_url = env::var("SERVICE_URL").expect("SERVICE_URL must be set");

    let mut app = App::new(Request::new(Client::new(), service_url));
    app.start().await;
}
