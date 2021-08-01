use serde_json::{Result, Value};
use std::{env, io};
use teloxide_core::{prelude::*, requests::Request, types::ChatId, Bot};

#[tokio::main]
async fn main() {
    let key: String = env::args().nth(1).expect("must provide API key as arg");
    let bot = Bot::new(key);

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Some((_, json)) = input.split_once(": ") {
                    input = json.to_string();
                }

                let val: Result<Value> = serde_json::from_str(input.as_str());
                match val {
                    Ok(v) => {
                        let mut send = false;
                        if v["lvl"] != "info" {
                            send = true;
                        }
                        if let Value::String(msg) = v["msg"].clone() {
                            if msg.contains("reorg") {
                                send = true;
                            }
                            if msg.contains("snap") {
                                send = false;
                            }
                        }
                        if send {
                            let req = bot.send_message(ChatId::from(-534327695), input.clone());
                            req.send().await.unwrap();
                        }
                    }
                    Err(e) => {
                        let msg = format!("parse error: {}\n\n{}", e, input);
                        let req = bot.send_message(ChatId::from(-534327695), msg);
                        req.send().await.unwrap();
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
