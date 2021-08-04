mod conditions;
mod handler;
mod matcher;
mod syslog;

use crate::{conditions::Condition, handler::handle_log, matcher::Matcher};
use std::{io, str::FromStr};
use structopt::StructOpt;
use teloxide_core::{
    requests::{Requester, RequesterExt},
    types::ChatId,
    Bot,
};

fn parse_matcher(src: &str) -> Matcher {
    let conditions: Vec<Condition> = src
        .split(",")
        .map(|c| match Condition::from_str(c) {
            Ok(c) => c,
            Err(e) => panic!("Unable to parse condition: {}", e),
        })
        .collect();

    Matcher::new(conditions)
}

fn parse_chat_id(src: &str) -> ChatId {
    ChatId::from(src.to_string())
}

#[derive(StructOpt)]
#[structopt(
    name = "parrot",
    about = "A simple CLI tool that parses go-ethereum logs from stdin and pipes them to another platform"
)]
struct Opt {
    /// Telegram bot API key
    #[structopt(long)]
    telegram_api_key: String,

    /// Telegram chat id where alerts should be sent
    #[structopt(long, parse(from_str = parse_chat_id))]
    telegram_chat_id: ChatId,

    #[structopt(long, parse(from_str = parse_matcher), default_value = "deny_info,reorg,deny_snap")]
    matcher: Matcher,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let bot = Bot::new(opt.telegram_api_key).auto_send();
    let mut input = String::new();

    loop {
        if io::stdin().read_line(&mut input).is_ok() {
            match handle_log(&input, &opt.matcher) {
                Ok(Some(msg)) => {
                    bot.send_message(opt.telegram_chat_id.clone(), msg).await?;
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
                _ => (),
            }
        }

        input.clear();
    }
}
