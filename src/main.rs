use anyhow::Result;
use log::info;
use pretty_env_logger::init;
use teloxide::Bot;

use crate::utils::json::read_json;

mod commands;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    info!("Starting up...");

    let bot = Bot::new(
        read_json("./secret/secret.json")?["token"]
            .as_str()
            .unwrap_or("__REPLACE_WITH_DEFAULT_TOKEN__"),
    );
    Ok(())
}
