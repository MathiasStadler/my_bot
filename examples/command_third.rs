use teloxide::{prelude::*, utils::command::BotCommands};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "my_bot * command_third => These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a ticker symbol.")]
    Symbol(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description = "repeat ticker symbol.")]
    Repeat,
}



async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is => @{username}."))
                .await?
        }
        Command::Symbol(symbol) => {
            bot.send_message(msg.chat.id, format!("The symbol is => @{symbol}."))
                .await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                msg.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
        Command::Repeat => {
            bot.send_message(msg.chat.id,format!("The repeat is active ."))
                .await?;
        
        //start
        loop{
            sleep(Duration::from_secs(10)).await;
            bot.send_message(msg.chat.id,format!("The repeat is active ."))
                .await?;

        }
        
        //end

        }
    };

    Ok(())
}
