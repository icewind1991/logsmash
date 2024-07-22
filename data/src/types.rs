use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, PartialEq, Clone, Copy, Deserialize)]
#[serde(from = "i64")]
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

impl From<i64> for LogLevel {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Debug,
            1 => Self::Info,
            2 => Self::Warn,
            3 => Self::Error,
            4 => Self::Critical,
            _ => Self::Unknown,
        }
    }
}

impl LogLevel {
    pub fn matches(&self, matcher_level: LogLevel) -> bool {
        matcher_level == *self || matcher_level == LogLevel::Exception || *self == LogLevel::Unknown
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoggingStatement {
    pub level: LogLevel,
    pub path: &'static str,
    pub line: usize,
    pub placeholders: &'static [&'static str],
    pub exception: Option<&'static str>,
    pub regex: &'static str,
}

impl LoggingStatement {
    pub fn message(&self) -> impl Display + '_ {
        LoggingMessage { message: &self }
    }
}

struct LoggingMessage<'a> {
    message: &'a LoggingStatement,
}

impl<'a> Display for LoggingMessage<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.message.regex.is_empty() {
            return Ok(());
        }
        let mut placeholder_index = 0;
        let regex = &self.message.regex[1..self.message.regex.len() - 1];
        for part in regex.split("(.*)") {
            write!(f, "{part}")?;
            if let Some(placeholder) = self.message.placeholders.get(placeholder_index) {
                write!(f, "{placeholder}")?;
            }
            placeholder_index += 1;
        }

        Ok(())
    }
}
