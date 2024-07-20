use databake::Bake;
use std::borrow::Cow;

#[derive(Debug, Default, PartialEq, Clone, Copy, Bake)]
#[databake(path = crate)]
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

impl From<crate::LogLevel> for LogLevel {
    fn from(value: crate::LogLevel) -> Self {
        match value {
            crate::LogLevel::Debug => LogLevel::Debug,
            crate::LogLevel::Info => LogLevel::Info,
            crate::LogLevel::Notice => LogLevel::Notice,
            crate::LogLevel::Warn => LogLevel::Warn,
            crate::LogLevel::Error => LogLevel::Error,
            crate::LogLevel::Alert => LogLevel::Alert,
            crate::LogLevel::Critical => LogLevel::Critical,
            crate::LogLevel::Emergency => LogLevel::Emergency,
            crate::LogLevel::Exception => LogLevel::Exception,
            crate::LogLevel::Unknown => LogLevel::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Bake)]
#[databake(path = crate)]
pub struct LoggingStatement<'a> {
    pub level: LogLevel,
    pub path: &'a str,
    pub line: usize,
    pub message_parts: &'a [&'a str],
}

pub fn bake_statement(output: &mut String, statement: &crate::LoggingStatement) {
    let message_parts: Vec<_> = statement.message_parts.iter().map(Cow::as_ref).collect();
    let statement = LoggingStatement {
        level: statement.level.into(),
        path: statement.path,
        line: statement.line,
        message_parts: &message_parts,
    };
    output.push_str(&statement.bake(&Default::default()).to_string());
}

#[cfg(feature = "bake")]
mod bake_test {
    #[test]
    fn test_bake() {
        use databake::test_bake;
        test_bake!(
            crate::LoggingStatement,
            const: crate::LoggingStatement {
                level: crate::LogLevel::Debug,
                path: "foo",
                line: 12usize,
                message_parts: &["part1", "part2"]
            },
            cloud_log_analyser,
        );
    }
}
