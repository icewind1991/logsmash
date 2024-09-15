use databake::Bake;

#[derive(Debug, Default, PartialEq, Clone, Copy, Bake)]
#[databake(path = crate)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
    Exception,
    #[default]
    Unknown,
}

impl From<crate::LogLevel> for LogLevel {
    fn from(value: crate::LogLevel) -> Self {
        match value {
            crate::LogLevel::Debug => LogLevel::Debug,
            crate::LogLevel::Info => LogLevel::Info,
            crate::LogLevel::Notice => LogLevel::Info,
            crate::LogLevel::Warn => LogLevel::Warn,
            crate::LogLevel::Error => LogLevel::Error,
            crate::LogLevel::Alert => LogLevel::Error,
            crate::LogLevel::Critical => LogLevel::Error,
            crate::LogLevel::Emergency => LogLevel::Error,
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
    pub placeholders: &'a [&'a str],
    pub exception: Option<&'a str>,
    pub pattern: &'a str,
    pub has_meaningful_message: bool,
}

fn build_pattern(parts: &[crate::MessagePart]) -> String {
    let mut pattern = String::with_capacity(128);
    for part in parts {
        match part {
            crate::MessagePart::Literal(literal) => pattern.push_str(literal.as_str()),
            crate::MessagePart::PlaceHolder(_placeholder) => {
                pattern.push('\0');
            }
        }
    }
    pattern.push_str("\x01\x01");
    pattern
}

pub fn bake_statement(output: &mut String, statement: &crate::LoggingStatement) {
    let placeholders: Vec<_> = statement
        .message_parts
        .iter()
        .filter_map(|part| match part {
            crate::MessagePart::PlaceHolder(placeholder) => Some(placeholder.as_str()),
            _ => None,
        })
        .collect();
    let pattern = build_pattern(&statement.message_parts);
    let statement = LoggingStatement {
        level: statement.level.into(),
        path: statement.path,
        line: statement.line,
        exception: statement.exception.as_deref(),
        placeholders: &placeholders,
        pattern: &pattern,
        has_meaningful_message: message_is_meaningful(&pattern),
    };
    output.push_str(&statement.bake(&Default::default()).to_string());
}

fn message_is_meaningful(msg: &str) -> bool {
    msg.chars().filter(|c| c.is_alphanumeric()).count() > 3
}
