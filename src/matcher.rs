use crate::logline::LogLine;
use cloud_log_analyser_data::{LogLevel, LoggingStatement, StatementList};
use itertools::Either;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::iter::once;

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
    pub fn new(statements: &StatementList) -> Matcher {
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
                    && exception.file.as_ref().ends_with(log_match.path)
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
    pub fn display<'a>(
        &'a self,
        log_statements: &'a StatementList,
        max_length: usize,
    ) -> impl Display + 'a {
        MatchResultDisplay {
            max_length,
            log_statements,
            result: &self,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            MatchResult::Single(_) => 1,
            MatchResult::List(list) => list.len(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        match self {
            MatchResult::Single(index) => Either::Left(once(*index)),
            MatchResult::List(list) => Either::Right(list.iter().copied()),
        }
    }
}

struct MatchResultDisplay<'a> {
    max_length: usize,
    log_statements: &'a StatementList,
    result: &'a MatchResult,
}

impl Display for MatchResultDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.result {
            MatchResult::Single(index) => {
                if let Some(statement) = self.log_statements.get(*index) {
                    write!(f, "{statement}")
                } else {
                    write!(f, "«unknown statement»")
                }
            }
            MatchResult::List(list) => {
                // todo: max length
                for (i, index) in list.iter().enumerate() {
                    if let Some(statement) = self.log_statements.get(*index) {
                        if i > 0 {
                            write!(f, "  or ")?;
                        }
                        write!(f, "{statement}\n")?;
                    }
                }
                Ok(())
            }
        }
    }
}

#[test]
fn test_matcher() {
    use crate::logline::Exception;

    const STATEMENTS: &[LoggingStatement] = &[
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
            exception: Some("Bar\\FooException"),
        },
    ];
    let matcher = Matcher::new(&StatementList::new(vec![("", STATEMENTS)]));
    assert_eq!(
        Some(MatchResult::Single(0)),
        matcher.match_log(&LogLine {
            version: "29",
            app: "core".into(),
            level: LogLevel::Error,
            message: "Not allowed to rename a shared album".into(),
            exception: None,
        })
    );
    assert_eq!(
        Some(MatchResult::List(vec![3, 4])),
        matcher.match_log(&LogLine {
            version: "29",
            app: "core".into(),
            level: LogLevel::Error,
            message: "Not allowed to rename an album".into(),
            exception: None,
        })
    );
    assert_eq!(
        Some(MatchResult::Single(1)),
        matcher.match_log(&LogLine {
            version: "29",
            app: "core".into(),
            level: LogLevel::Error,
            message: "You are not allowed to edit link shares that you don't own".into(),
            exception: None,
        })
    );
    assert_eq!(
        None,
        matcher.match_log(&LogLine {
            version: "29",
            app: "core".into(),
            level: LogLevel::Info,
            message: "You are not allowed to edit link shares that you don't own".into(),
            exception: None,
        })
    );
    assert_eq!(
        Some(MatchResult::Single(2)),
        matcher.match_log(
            &LogLine {
                version: "29",
                app: "core".into(),
                level:  LogLevel::Error,
                message: "Unsupported query value for mimetype: %/text, only values in the format \"mime/type\" or \"mime/%\" are supported".into(),
                exception: None,
            }
        )
    );
    assert_eq!(
        Some(MatchResult::Single(4)),
        matcher.match_log(
            &LogLine {
                version: "29",
                app: "core".into(),
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
