use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, PartialEq, Clone, Copy, Deserialize, Hash, PartialOrd, Ord, Eq)]
#[serde(from = "i64")]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
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
    pub fn iter() -> impl Iterator<Item = LogLevel> {
        [
            LogLevel::Debug,
            LogLevel::Info,
            LogLevel::Warn,
            LogLevel::Error,
            LogLevel::Exception,
            LogLevel::Unknown,
        ]
        .into_iter()
    }

    pub fn matches(&self, matcher_level: LogLevel) -> bool {
        matcher_level == *self || matcher_level == LogLevel::Exception || *self == LogLevel::Unknown
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
            LogLevel::Exception => "exception",
            LogLevel::Unknown => "log",
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoggingStatement {
    pub level: LogLevel,
    pub path: &'static str,
    pub line: usize,
    pub placeholders: &'static [&'static str],
    pub exception: Option<&'static str>,
    pub pattern: &'static str,
    pub has_meaningful_message: bool,
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
            pattern: self.pattern,
            has_meaningful_message: self.has_meaningful_message,
        }
    }

    pub fn message(&self) -> impl Display + '_ {
        LoggingMessage {
            message: self.clone(),
        }
    }

    pub fn pattern_len(&self) -> usize {
        self.pattern.len()
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
    pub pattern: &'static str,
    pub has_meaningful_message: bool,
}

impl From<&LoggingStatementWithPathPrefix> for LoggingStatement {
    fn from(value: &LoggingStatementWithPathPrefix) -> Self {
        LoggingStatement {
            level: value.level,
            path: value.path,
            line: value.line,
            placeholders: value.placeholders,
            exception: value.exception,
            pattern: value.pattern,
            has_meaningful_message: value.has_meaningful_message,
        }
    }
}

impl LoggingStatementWithPathPrefix {
    fn raw_message(&self) -> LoggingMessage {
        LoggingMessage {
            message: self.into(),
        }
    }

    pub fn path(&self) -> impl Display {
        LoggingStatementPath {
            path_prefix: self.path_prefix,
            path: self.path,
        }
    }

    pub fn message(&self) -> impl Display {
        LoggingStatementMessage {
            message: self.raw_message(),
            exception: self.exception,
        }
    }
}

struct LoggingStatementPath {
    pub path_prefix: &'static str,
    pub path: &'static str,
}

impl Display for LoggingStatementPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.path_prefix, self.path,)
    }
}

struct LoggingStatementMessage {
    pub message: LoggingMessage,
    pub exception: Option<&'static str>,
}

impl Display for LoggingStatementMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(exception) = self.exception {
            write!(f, "{}({})", exception, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl Display for LoggingStatementWithPathPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "«{}» {} line {}",
            self.raw_message(),
            self.path(),
            self.line
        )
    }
}

struct LoggingMessage {
    message: LoggingStatement,
}

impl Display for LoggingMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.message.pattern.is_empty() {
            return Ok(());
        }
        let mut placeholder_index = 0;
        for part in self.message.pattern.trim_end_matches('\x01').split('\0') {
            write!(f, "{part}")?;
            if let Some(placeholder) = self.message.placeholders.get(placeholder_index) {
                write!(f, "{placeholder}")?;
            }
            placeholder_index += 1;
        }

        Ok(())
    }
}
