use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
            "printErrorPage" => Some(LogLevel::Unknown),
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

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
