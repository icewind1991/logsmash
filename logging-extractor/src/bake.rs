use databake::Bake;

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
    pub placeholders: &'a [&'a str],
    pub has_meaningful_message: bool,
    pub exception: Option<&'a str>,
    pub regex: &'a str,
}

fn build_pattern(parts: &[crate::MessagePart]) -> String {
    let mut pattern = String::with_capacity(128);
    pattern.push('^');
    for part in parts {
        match part {
            crate::MessagePart::Literal(literal) => {
                pattern.push_str(&regex_syntax::escape(literal))
            }
            crate::MessagePart::PlaceHolder(_placeholder) => {
                pattern.push_str("(.*)");
            }
        }
    }
    pattern.push('$');
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
        has_meaningful_message: statement.has_meaningful_message,
        exception: statement.exception.as_deref(),
        placeholders: &placeholders,
        regex: &pattern,
    };
    output.push_str(&statement.bake(&Default::default()).to_string());
}
