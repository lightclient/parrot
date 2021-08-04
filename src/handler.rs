use crate::matcher::Matcher;
use crate::syslog::Severity;
use serde_json::Value;
use std::error::Error;
use std::str::FromStr;
use syslog_loose::Message;

pub fn handle_log(s: &str, m: &Matcher) -> Result<Option<String>, Box<dyn Error>> {
    let log: Message<&str> = syslog_loose::parse_message(s);

    match serde_json::from_str::<Value>(&log.msg) {
        Ok(v) => match m.matches(&v) {
            true => Ok(Some(format_alert(v))),
            false => Ok(None),
        },
        Err(e) => Ok(Some(format!("parse error: {}\n\n{}", e, s).to_string())),
    }
}

fn format_alert(v: Value) -> String {
    let lvl = Severity::from_str(v["lvl"].as_str().unwrap()).unwrap();
    let msg = v["msg"].as_str().unwrap();

    let mut extra = String::new();
    if let Some(obj) = v.as_object() {
        for (k, v) in obj {
            match k.as_str() {
                "msg" | "lvl" | "t" | "dropfrom" | "addfrom" => continue,
                _ => extra.push_str(format!("{}={}\t", k, v).as_str()),
            }
        }
    }

    format!("{}: {}\n\n{}", lvl, msg, extra)
}
