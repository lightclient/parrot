use crate::conditions::Condition;
use serde_json::Value;

pub struct Matcher {
    conditions: Vec<Condition>,
}

impl Matcher {
    pub fn new(conditions: Vec<Condition>) -> Self {
        Self { conditions }
    }

    pub fn matches(&self, s: &Value) -> bool {
        self.conditions
            .iter()
            .fold(true, |last, condition| match condition.check(s) {
                Some(curr) => curr,
                None => last,
            })
    }
}

mod tests {
    #![allow(dead_code)]
    use super::*;

    #[allow(unused_macros)]
    macro_rules! conditions {
        ($($x:expr),+ $(,)?) => (
            vec![$(Condition(Box::new($x))),+]
        );
    }

    fn accept(_: &Value) -> Option<bool> {
        Some(true)
    }

    fn reject(_: &Value) -> Option<bool> {
        Some(false)
    }

    fn pass(_: &Value) -> Option<bool> {
        None
    }

    #[test]
    fn simple_test() {
        let conditions: Vec<Condition> = conditions![accept, reject, pass];
        let alerter = Matcher::new(conditions);
        assert_eq!(alerter.matches(&Value::String("".into())), false);
    }

    #[test]
    fn more_complicated() {
        let conditions: Vec<Condition> =
            conditions![pass, pass, accept, reject, pass, accept, pass];
        let alerter = Matcher::new(conditions);
        assert_eq!(alerter.matches(&Value::String("".into())), true);
    }
}
