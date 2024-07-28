use crate::logline::LogLine;
use itertools::Either;
use logsmash_data::{LogLevel, LoggingStatement, StatementList};
use rayon::prelude::*;
use regex::Regex;
use std::iter::once;

pub struct LogMatch {
    level: LogLevel,
    pattern: Regex,
    pattern_length: usize,
    has_meaningful_message: bool,
    exception: Option<&'static str>,
    path: &'static str,
    line: usize,
    index: usize,
}

impl LogMatch {
    pub fn new(index: usize, statement: &LoggingStatement) -> LogMatch {
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
            index,
        }
    }
}

pub struct Matcher {
    matches: Vec<LogMatch>,
}

impl Matcher {
    pub fn new(statements: &StatementList) -> Matcher {
        let matches: Vec<_> = statements
            .iter()
            .enumerate()
            .par_bridge()
            .map(|(index, statement)| LogMatch::new(index, statement))
            .collect();

        Matcher { matches }
    }

    pub fn match_log(&self, log: &LogLine) -> Option<MatchResult> {
        let mut best_match = None;
        let mut best_length = 0;

        if let Some(exception) = &log.exception {
            for log_match in self.matches.iter() {
                if log_match.line == exception.line
                    && log_match.exception == Some(exception.exception.as_str())
                    && exception.file.ends_with(log_match.path)
                {
                    return Some(MatchResult::Single(log_match.index));
                }
            }
        }

        for log_match in self.matches.iter() {
            if log_match.has_meaningful_message
                && log.level.matches(log_match.level)
                && log_match.pattern.is_match(log.message.as_str())
                && log_match.pattern_length >= best_length
            {
                if log_match.pattern_length > best_length {
                    best_match = None;
                    best_length = log_match.pattern_length;
                }
                best_match = Some(match best_match {
                    Some(MatchResult::Single(res)) => MatchResult::List(vec![res, log_match.index]),
                    Some(MatchResult::List(mut list)) => {
                        list.push(log_match.index);
                        MatchResult::List(list)
                    }
                    None => MatchResult::Single(log_match.index),
                });
            }
        }

        // todo: handle translated log messages

        best_match
    }
}

#[derive(Debug, Clone, Eq, Hash)]
pub enum MatchResult {
    Single(usize),
    List(Vec<usize>),
}

impl PartialEq for MatchResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MatchResult::Single(a), MatchResult::Single(b)) => a.eq(b),
            (MatchResult::List(a), MatchResult::List(b)) => {
                let mut a = a.clone();
                let mut b = b.clone();
                a.sort();
                b.sort();
                a.eq(&b)
            }
            _ => false,
        }
    }
}

impl MatchResult {
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

#[test]
fn test_matcher() {
    use crate::logline::Exception;
    use time::OffsetDateTime;
    use tinystr::TinyAsciiStr;

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
            version: TinyAsciiStr::from_str("29").unwrap(),
            app: TinyAsciiStr::from_str("core").unwrap(),
            level: LogLevel::Error,
            message: "Not allowed to rename a shared album".into(),
            exception: None,
            time: OffsetDateTime::now_utc(),
            index: 0,
        })
    );
    assert_eq!(
        Some(MatchResult::List(vec![3, 4])),
        matcher.match_log(&LogLine {
            version: TinyAsciiStr::from_str("29").unwrap(),
            app: TinyAsciiStr::from_str("core").unwrap(),
            level: LogLevel::Error,
            message: "Not allowed to rename an album".into(),
            exception: None,
            time: OffsetDateTime::now_utc(),
            index: 0,
        })
    );
    assert_eq!(
        Some(MatchResult::Single(1)),
        matcher.match_log(&LogLine {
            version: TinyAsciiStr::from_str("29").unwrap(),
            app: TinyAsciiStr::from_str("core").unwrap(),
            level: LogLevel::Error,
            message: "You are not allowed to edit link shares that you don't own".into(),
            exception: None,
            time: OffsetDateTime::now_utc(),
            index: 0,
        })
    );
    assert_eq!(
        None,
        matcher.match_log(&LogLine {
            version: TinyAsciiStr::from_str("29").unwrap(),
            app: TinyAsciiStr::from_str("core").unwrap(),
            level: LogLevel::Info,
            message: "You are not allowed to edit link shares that you don't own".into(),
            exception: None,
            time: OffsetDateTime::now_utc(),
            index: 0,
        })
    );
    assert_eq!(
        Some(MatchResult::Single(2)),
        matcher.match_log(
            &LogLine {
                version: TinyAsciiStr::from_str("29").unwrap(),
                app: TinyAsciiStr::from_str("core").unwrap(),
                level:  LogLevel::Error,
                message: "Unsupported query value for mimetype: %/text, only values in the format \"mime/type\" or \"mime/%\" are supported".into(),
                exception: None,
                time: OffsetDateTime::now_utc(),
                index: 0,
            }
        )
    );
    assert_eq!(
        Some(MatchResult::Single(4)),
        matcher.match_log(
            &LogLine {
                version: TinyAsciiStr::from_str("29").unwrap(),
                app: TinyAsciiStr::from_str("core").unwrap(),
                level:  LogLevel::Error,
                message: "Unsupported query value for mimetype: %/text, only values in the format \"mime/type\" or \"mime/%\" are supported".into(),
                exception: Some(Exception {
                    message: "".into(),
                    exception: "Bar\\FooException".into(),
                    file: "short".into(),
                    line: 68,
                    previous: None,
                }),
                time: OffsetDateTime::now_utc(),
                index: 0,
            }
        )
    );
}
