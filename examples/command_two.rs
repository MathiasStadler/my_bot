// tokio repeat fn call
// https://stackoverflow.com/questions/66863385/how-can-i-use-tokio-to-trigger-a-function-every-period-or-interval-in-seconds

use std::time::Duration;
use teloxide::{prelude::*, utils::command::BotCommands};
// use tokio::{task, time}; // 1.3.0

// use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    // REPL ("read-eval-print loop")
    Command::repl(bot, answer).await;

    let val = std::cell::Cell::new(1);
    tokio::select! {
      _ = async {loop {
        println!(".{}", val.get());
        sleep(Duration::from_secs(5)).await;
        println!("{}","hallo");
      }} => {},
    }
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a ticker symbol.")]
    Symbol(String),
    #[command(description = "repeat the command fix time period.")]
    Repeat,
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description = "handle a username and an age.", parse_with = "split")]
    RepeatEach { each: u8 },
}

#[allow(dead_code)]
#[allow(unused_variables)]
async fn send_repeat_msg(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    bot.send_message(
        msg.chat.id,
        format!("TEST repeat msg => Your username is => username."),
    )
    .await?;
    Ok(())
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
            bot.send_message(msg.chat.id, format!("Repeat ticker"))
                .await?
        }
        Command::RepeatEach { each } => {
            bot.send_message(msg.chat.id, format!("Repeat ticker @{each} "))
                .await?
        }
    };

    Ok(())
}
