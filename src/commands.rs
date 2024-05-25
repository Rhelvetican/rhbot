use teloxide::{macros::BotCommands, types::Message, Bot};

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Commands {
    #[command(description = "display this text.")]
    Help,
}

pub async fn reply(bot: Bot, msg: Message, cmd: Commands) {
    match cmd {
        Commands::Help => msg.reply_to_message(),
    }
}
