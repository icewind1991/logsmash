#[derive(Debug, Default, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct LoggingStatement {
    pub level: LogLevel,
    pub path: &'static str,
    pub line: usize,
    pub placeholders: &'static [&'static str],
    pub regex: &'static str,
}
