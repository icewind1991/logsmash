use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Notice,
    Warn,
    Error,
    Alert,
    Critical,
    Emergency,
    Exception,
    #[default]
    Unknown,
}

impl LogLevel {
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "debug" => Some(LogLevel::Debug),
            "info" => Some(LogLevel::Info),
            "notice" => Some(LogLevel::Notice),
            "warn" | "warning" => Some(LogLevel::Warn),
            "error" => Some(LogLevel::Error),
            "alert" => Some(LogLevel::Alert),
            "critical" => Some(LogLevel::Critical),
            "emergency" => Some(LogLevel::Emergency),
            "exception" => Some(LogLevel::Exception),
            "log" => Some(LogLevel::Unknown),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Notice => "notice",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
            LogLevel::Alert => "alert",
            LogLevel::Critical => "critical",
            LogLevel::Emergency => "emergency",
            LogLevel::Exception => "exception",
            LogLevel::Unknown => "log",
        }
    }
}

impl Serialize for LogLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        Ok(LogLevel::parse(s).unwrap_or_default())
    }
}
