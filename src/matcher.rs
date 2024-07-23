use crate::logline::LogLine;
use cloud_log_analyser_data::{LogLevel, LoggingStatement};
use regex::Regex;
use std::fmt::{Display, Formatter};

pub struct LogMatch {
    level: LogLevel,
    pattern: Regex,
    pattern_length: usize,
    has_meaningful_message: bool,
    exception: Option<&'static str>,
    path: &'static str,
    line: usize,
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
            path: statement.path,
            line: statement.line,
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

    pub fn match_log(&self, log: &LogLine) -> Option<MatchResult> {
        let mut best_match = None;
        let mut best_length = 0;

        if let Some(exception) = &log.exception {
            for (i, log_match) in self.matches.iter().enumerate() {
                if log_match.line == exception.line
                    && log_match.exception.as_deref() == Some(exception.exception.as_ref())
                    && log_match.path == exception.file.as_ref()
                {
                    return Some(MatchResult::Single(i));
                }
            }
        }

        for (i, log_match) in self.matches.iter().enumerate() {
            if log_match.has_meaningful_message {
                if log.level.matches(log_match.level)
                    && log_match.pattern.is_match(log.message.as_ref())
                    && log_match.pattern_length >= best_length
                {
                    if log_match.pattern_length > best_length {
                        best_match = None;
                        best_length = log_match.pattern_length;
                    }
                    best_match = Some(match best_match {
                        Some(MatchResult::Single(res)) => MatchResult::List(vec![res, i]),
                        Some(MatchResult::List(mut list)) => {
                            list.push(i);
                            MatchResult::List(list)
                        }
                        None => MatchResult::Single(i),
                    });
                }
            }
        }

        // todo: handle translated log messages

        best_match
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MatchResult {
    Single(usize),
    List(Vec<usize>),
}

impl MatchResult {
    pub fn display<'a>(&'a self, log_statements: &'a [LoggingStatement]) -> impl Display + 'a {
        MatchResultDisplay {
            log_statements,
            result: &self,
        }
    }
}

struct MatchResultDisplay<'a> {
    log_statements: &'a [LoggingStatement],
    result: &'a MatchResult,
}

impl Display for MatchResultDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.result {
            MatchResult::Single(index) => {
                if let Some(statement) = self.log_statements.get(*index) {
                    write!(f, "{statement}")
                } else {
                    write!(f, "unknown statement")
                }
            }
            MatchResult::List(list) => {
                writeln!(f, "{} possible matches:", list.len())?;
                for index in list {
                    if let Some(statement) = self.log_statements.get(*index) {
                        writeln!(f, "  {statement}")?;
                    }
                }
                write!(f, "    ")
            }
        }
    }
}

#[test]
fn test_matcher() {
    use crate::logline::Exception;

    let statements = &[
        LoggingStatement {
            line: 68,
            level: LogLevel::Exception,
            path: "foo",
            placeholders: &[],
            regex: "^Not allowed to rename a shared album$",
            exception: None,
        },
        LoggingStatement {
            line: 69,
            level: LogLevel::Error,
            path: "bar",
            placeholders: &[],
            regex: "^You are not allowed to edit link shares that you don't own$",
            exception: None,
        },
        LoggingStatement {
            line: 69,
            level: LogLevel::Error,
            path: "asd",
            placeholders: &["$mimeType"],
            regex: r#"^Unsupported query value for mimetype: (.*), only values in the format "mime/type" or "mime/%" are supported$"#,
            exception: None,
        },
        LoggingStatement {
            line: 68,
            level: LogLevel::Exception,
            path: "short",
            placeholders: &["$path"],
            regex: "^Not allowed to rename (.*)$",
            exception: None,
        },
        LoggingStatement {
            line: 68,
            level: LogLevel::Exception,
            path: "short",
            placeholders: &["$path"],
            regex: "^Not allowed to rename (.*)$",
            exception: "Bar\\FooException".into(),
        },
    ];
    let matcher = Matcher::new(statements);
    assert_eq!(
        Some(0),
        matcher.match_log(&LogLine {
            version: "29",
            level: LogLevel::Error,
            message: "Not allowed to rename a shared album".into(),
            exception: None,
        })
    );
    assert_eq!(
        Some(3),
        matcher.match_log(&LogLine {
            version: "29",
            level: LogLevel::Error,
            message: "Not allowed to rename an album".into(),
            exception: None,
        })
    );
    assert_eq!(
        Some(1),
        matcher.match_log(&LogLine {
            version: "29",
            level: LogLevel::Error,
            message: "You are not allowed to edit link shares that you don't own".into(),
            exception: None,
        })
    );
    assert_eq!(
        None,
        matcher.match_log(&LogLine {
            version: "29",
            level: LogLevel::Info,
            message: "You are not allowed to edit link shares that you don't own".into(),
            exception: None,
        })
    );
    assert_eq!(
        Some(2),
        matcher.match_log(
            &LogLine {
                version: "29",
                level:  LogLevel::Error,
                message: "Unsupported query value for mimetype: %/text, only values in the format \"mime/type\" or \"mime/%\" are supported".into(),
                exception: None,
            }
        )
    );
    assert_eq!(
        Some(4),
        matcher.match_log(
            &LogLine {
                version: "29",
                level:  LogLevel::Error,
                message: "Unsupported query value for mimetype: %/text, only values in the format \"mime/type\" or \"mime/%\" are supported".into(),
                exception: Some(Exception {
                    exception: "Bar\\FooException".into(),
                    file: "short".into(),
                    line: 68,
                    previous: None,
                }),
            }
        )
    );
}
