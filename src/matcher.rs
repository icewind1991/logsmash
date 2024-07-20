use cloud_log_analyser_data::{LogLevel, LoggingStatement};
use regex::{escape, Regex, RegexBuilder};

pub struct LogMatch {
    level: LogLevel,
    pattern: Regex,
    pattern_length: usize,
}

impl LogMatch {
    pub fn new(statement: &LoggingStatement) -> LogMatch {
        LogMatch {
            level: statement.level,
            pattern: build_pattern(statement.message_parts),
            pattern_length: statement.message_parts.iter().copied().map(str::len).sum(),
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

        best_match
    }
}

fn build_pattern<'a>(parts: &[&str]) -> Regex {
    let mut pattern = String::with_capacity(128);
    for part in parts {
        pattern.push_str(&escape(part));
        pattern.push_str("(.*)");
    }
    RegexBuilder::new(&pattern)
        .build()
        .expect("Failed to build regex")
}

#[test]
fn test_build_pattern() {
    let regex = build_pattern(["foobar", "asd"]);
    assert!(regex.is_match("foobar with asd and more"));
    assert!(regex.is_match("foobarasd"));
    assert!(!regex.is_match("fooasd"));
}

#[test]
fn test_matcher() {
    let statements = &[
        LoggingStatement {
            line: 68,
            level: LogLevel::Exception,
            path: "foo",
            message_parts: vec!["Not allowed to rename a shared album".into()],
        },
        LoggingStatement {
            line: 69,
            level: LogLevel::Error,
            path: "bar",
            message_parts: vec![
                "You are not allowed to edit link shares that you don".into(),
                "'".into(),
                "t own".into(),
            ],
        },
        LoggingStatement {
            line: 69,
            level: LogLevel::Error,
            path: "asd",
            message_parts: vec![
                "Unsupported query value for mimetype: ".into(),
                ", only values in the format \"mime/type\" or \"mime/%\" are supported".into(),
            ],
        },
        LoggingStatement {
            line: 68,
            level: LogLevel::Exception,
            path: "short",
            message_parts: vec!["Not allowed to rename".into()],
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
