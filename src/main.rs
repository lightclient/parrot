mod syslog;

use crate::syslog::Severity;
use serde_json::Value;
use std::error::Error;
use std::io;
use std::str::FromStr;
use structopt::StructOpt;
use syslog_loose::Message;
use teloxide_core::{
    requests::{Requester, RequesterExt},
    types::ChatId,
    Bot,
};

fn parse_chat_id(src: &str) -> ChatId {
    ChatId::from(src.to_string())
}

#[derive(Debug, StructOpt)]
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let bot = Bot::new(opt.telegram_api_key).auto_send();
    let mut input = String::new();

    loop {
        if io::stdin().read_line(&mut input).is_ok() {
            match handle_log(&input) {
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

fn handle_log(s: &str) -> Result<Option<String>, Box<dyn Error>> {
    let log: Message<&str> = syslog_loose::parse_message(s);

    match serde_json::from_str::<Value>(&log.msg) {
        Ok(v) => {
            let mut send = false;

            if let Value::String(msg) = v["msg"].clone() {
                if msg.contains("reorg") {
                    send = true;
                }
                if msg.contains("snap") {
                    send = false;
                }
            }
            if send {
                Ok(Some(format_alert(v)))
            } else {
                Ok(None)
            }
        }
        Err(e) => Ok(Some(format!("parse error: {}\n\n{}", e, s).to_string())),
    }
}

fn format_alert(v: Value) -> String {
    let lvl = Severity::from_str(v["lvl"].as_str().unwrap()).unwrap();
    let msg = v["msg"].as_str().unwrap();

    format!("{}: {}", lvl, msg)
}
