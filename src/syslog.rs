use std::fmt;
use std::str::FromStr;

pub enum Severity {
    Critical,
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

impl FromStr for Severity {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "crit" => Ok(Self::Critical),
            "eror" => Ok(Self::Error),
            "warn" => Ok(Self::Warning),
            "info" => Ok(Self::Info),
            "dbug" => Ok(Self::Debug),
            "trce" => Ok(Self::Trace),
            other => Err(format!("unknown severity level: {}", other).into()),
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Self::Critical => "CRITICAL",
            Self::Error => "ERROR",
            Self::Warning => "WARNING",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        };
        write!(f, "{}", out)
    }
}
