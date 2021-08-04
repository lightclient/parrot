use serde_json::Value;
use std::str::FromStr;

pub struct Condition(pub(crate) Box<dyn Fn(&Value) -> Option<bool>>);

impl Condition {
    pub fn check(&self, v: &Value) -> Option<bool> {
        self.0(v)
    }
}

impl FromStr for Condition {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = match s {
            "deny_info" => is_priority_info(false),
            "reorg" => contains_reorg(3, true),
            _ => {
                if let Some(val) = s.strip_prefix("deny_") {
                    contains_string(val.to_string(), false)
                } else if let Some(val) = s.strip_prefix("allow_") {
                    contains_string(val.to_string(), true)
                } else {
                    unimplemented!()
                }
            }
        };
        Ok(f)
    }
}

pub fn contains_string(s: String, accept: bool) -> Condition {
    let f = move |v: &Value| {
        if let Value::String(msg) = v["msg"].clone() {
            if msg.contains(&s) {
                return Some(accept);
            }
        }

        None
    };

    Condition(Box::new(f))
}

pub fn contains_reorg(count: u8, accept: bool) -> Condition {
    let f = move |v: &Value| {
        if let Value::String(msg) = v["msg"].clone() {
            if msg.contains("reorg") && u8::from_str(v["drop"].as_str().unwrap()).unwrap() >= count
            {
                return Some(accept);
            }
        }

        None
    };

    Condition(Box::new(f))
}

pub fn is_priority_info(accept: bool) -> Condition {
    let f = move |v: &Value| {
        if v["lvl"] == "info" {
            return Some(accept);
        } else {
            None
        }
    };

    Condition(Box::new(f))
}
