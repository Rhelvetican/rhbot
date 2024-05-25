use log::info;
use pretty_env_logger::init;
use teloxide::Bot;

mod commands;
mod utils;

#[tokio::main]
async fn main() {
    init();
    info!("Starting up...");

    let bot = Bot::new(token);
}
