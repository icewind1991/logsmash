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
            _ => Self::Unknown,
        }
    }
}

impl LogLevel {
    pub fn matches(&self, matcher_level: LogLevel) -> bool {
        let matcher_level = match matcher_level {
            LogLevel::Notice => LogLevel::Info,
            LogLevel::Alert | LogLevel::Critical | LogLevel::Emergency => LogLevel::Error,
            _ => matcher_level,
        };
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
    pub fn with_path_prefix(&self, path_prefix: &'static str) -> LoggingStatementWithPathPrefix {
        LoggingStatementWithPathPrefix {
            level: self.level,
            path_prefix,
            path: self.path,
            line: self.line,
            placeholders: self.placeholders,
            exception: self.exception,
            regex: self.regex,
        }
    }

    pub fn message(&self) -> impl Display + '_ {
        LoggingMessage {
            message: self.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoggingStatementWithPathPrefix {
    pub level: LogLevel,
    pub path_prefix: &'static str,
    pub path: &'static str,
    pub line: usize,
    pub placeholders: &'static [&'static str],
    pub exception: Option<&'static str>,
    pub regex: &'static str,
}

impl From<&LoggingStatementWithPathPrefix> for LoggingStatement {
    fn from(value: &LoggingStatementWithPathPrefix) -> Self {
        LoggingStatement {
            level: value.level,
            path: value.path,
            line: value.line,
            placeholders: value.placeholders,
            exception: value.exception,
            regex: value.regex,
        }
    }
}

impl Display for LoggingStatementWithPathPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(exception) = self.exception {
            write!(
                f,
                "{}({}): {}{} line {}",
                exception,
                self.message(),
                self.path_prefix,
                self.path,
                self.line
            )
        } else {
            write!(
                f,
                "{}: {}{} line {}",
                self.message(),
                self.path_prefix,
                self.path,
                self.line
            )
        }
    }
}

impl LoggingStatementWithPathPrefix {
    pub fn message(&self) -> impl Display + '_ {
        LoggingMessage {
            message: self.into(),
        }
    }
}

struct LoggingMessage {
    message: LoggingStatement,
}

impl Display for LoggingMessage {
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
