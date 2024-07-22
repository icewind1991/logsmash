use cloud_log_analyser_data::{LogLevel, LoggingStatement};
use regex::Regex;

pub struct LogMatch {
    level: LogLevel,
    pattern: Regex,
    pattern_length: usize,
    has_meaningful_message: bool,
    exception: Option<&'static str>,
}

impl LogMatch {
    pub fn new(statement: &LoggingStatement) -> LogMatch {
        LogMatch {
            level: statement.level,
            pattern: Regex::new(statement.regex).unwrap(),
            pattern_length: statement.regex.len(),
            has_meaningful_message: statement
                .regex
                .contains(|c: char| c.is_ascii_alphanumeric()),
            exception: statement.exception,
        }
    }
}

pub struct Matcher {
    matches: Vec<LogMatch>,
}

impl Matcher {
    pub fn new(statements: &[LoggingStatement]) -> Matcher {
        Matcher {
            matches: statements.iter().map(LogMatch::new).collect(),
        }
    }

    pub fn match_log(&self, level: LogLevel, message: &str) -> Option<usize> {
        let mut best_match = None;
        let mut best_length = 0;

        for (i, log_match) in self.matches.iter().enumerate() {
            if log_match.has_meaningful_message {
                if (log_match.level == level
                    || log_match.level == LogLevel::Exception
                    || level == LogLevel::Unknown)
                    && log_match.pattern.is_match(message)
                    && log_match.pattern_length > best_length
                {
                    best_match = Some(i);
                    best_length = log_match.pattern_length;
                }
            }
        }

        best_match
    }
}

#[test]
fn test_matcher() {
    let statements = &[
        LoggingStatement {
            line: 68,
            level: LogLevel::Exception,
            path: "foo",
            placeholders: &[],
            regex: "^Not allowed to rename a shared album$",
        },
        LoggingStatement {
            line: 69,
            level: LogLevel::Error,
            path: "bar",
            placeholders: &[],
            regex: "^You are not allowed to edit link shares that you don't own$",
        },
        LoggingStatement {
            line: 69,
            level: LogLevel::Error,
            path: "asd",
            placeholders: &["$mimeType"],
            regex: r#"^Unsupported query value for mimetype: (.*), only values in the format "mime/type" or "mime/%" are supported$"#,
        },
        LoggingStatement {
            line: 68,
            level: LogLevel::Exception,
            path: "short",
            placeholders: &["$path"],
            regex: "^Not allowed to rename (.*)$",
        },
    ];
    let matcher = Matcher::new(statements);
    assert_eq!(
        Some(0),
        matcher.match_log(LogLevel::Error, "Not allowed to rename a shared album")
    );
    assert_eq!(
        Some(3),
        matcher.match_log(LogLevel::Error, "Not allowed to rename an album")
    );
    assert_eq!(
        Some(1),
        matcher.match_log(
            LogLevel::Error,
            "You are not allowed to edit link shares that you don't own"
        )
    );
    assert_eq!(
        None,
        matcher.match_log(
            LogLevel::Info,
            "You are not allowed to edit link shares that you don't own"
        )
    );
    assert_eq!(Some(2), matcher.match_log(LogLevel::Error, "Unsupported query value for mimetype: %/text, only values in the format \"mime/type\" or \"mime/%\" are supported"));
}
