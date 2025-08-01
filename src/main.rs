use std::time::Instant;

use teloxide::{Bot, prelude::*, utils::command::BotCommands};

use greenrock::{
    processor::load_btc_data,
    strategy::{
        core::{MinimalStrategy, Strategy},
        utils::row_to_kline,
    },
};
use polars::prelude::{IntoLazy, col};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let df = load_btc_data("processed_btc_data/2025-01.parquet");

    let mut strategy = MinimalStrategy::new(df.clone());

    let df = df
        .clone()
        .lazy()
        .select([
            col("timestamp"),
            col("open"),
            col("high"),
            col("low"),
            col("close"),
            col("volume"),
        ])
        .collect()
        .unwrap();

    let start = Instant::now();

    for i in 0..df.shape().0 {
        let tick = row_to_kline(&df, i);
        strategy.state = strategy.tick(&mut strategy.state.clone(), Some(tick));
    }

    println!(
        "evaluated {} klines in {:.3}s",
        df.shape().0,
        start.elapsed().as_secs_f64()
    );

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
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
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                .await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                msg.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
    };

    Ok(())
}
