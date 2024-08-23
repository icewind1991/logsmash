use crate::logline::{Exception, LogLine};
use itertools::Either;
use logsmash_data::{LogLevel, LoggingStatement, StatementList};
use std::hash::{Hash, Hasher};
use std::iter::once;

pub struct LogMatch {
    level: LogLevel,
    pattern: Option<&'static str>,
    exception: Option<&'static str>,
    path: &'static str,
    line: usize,
    index: usize,
}

impl LogMatch {
    pub fn new(index: usize, statement: &LoggingStatement) -> LogMatch {
        LogMatch {
            level: statement.level,
            pattern: statement
                .has_meaningful_message()
                .then_some(statement.pattern),
            exception: statement.exception,
            path: statement.path,
            line: statement.line,
            index,
        }
    }

    pub fn pattern_len(&self) -> usize {
        self.pattern.map(|pat| pat.len()).unwrap_or_default()
    }
}

pub struct Matcher {
    matches: Vec<LogMatch>,
}

impl Matcher {
    pub fn new(statements: &StatementList) -> Matcher {
        let mut matches: Vec<_> = statements
            .iter()
            .enumerate()
            .map(|(index, statement)| LogMatch::new(index, statement))
            .collect();
        matches.sort_by(|a, b| a.pattern_len().cmp(&b.pattern_len()).reverse());

        Matcher { matches }
    }

    pub fn match_log(&self, log: &LogLine) -> Option<MatchResult> {
        let mut best_match = None;
        let mut best_length = 0;

        if let Some(exception) = &log.exception {
            if let Some(index) = self.match_exception(exception) {
                return Some(MatchResult::Single(index));
            }
        }

        for log_match in self.matches.iter() {
            if log_match.pattern_len() < best_length {
                break;
            }

            if let Some(source_pattern) = log_match.pattern {
                if log.level.matches(log_match.level) {
                    if match_single(source_pattern, log.message.as_ref()) {
                        best_length = log_match.pattern_len();
                        best_match = Some(match best_match {
                            Some(MatchResult::Single(res)) => {
                                MatchResult::List(vec![res, log_match.index])
                            }
                            Some(MatchResult::List(mut list)) => {
                                list.push(log_match.index);
                                MatchResult::List(list)
                            }
                            None => MatchResult::Single(log_match.index),
                        });
                    }
                }
            }
        }

        // todo: handle translated log messages

        best_match
    }

    fn match_exception(&self, exception: &Exception) -> Option<usize> {
        for log_match in self.matches.iter() {
            if log_match.line == exception.line
                && log_match.exception == Some(exception.exception.as_ref())
                && exception.file.ends_with(log_match.path)
            {
                return Some(log_match.index);
            }
        }
        None
    }
}

pub fn match_single(pattern: &str, text: &str) -> bool {
    let mut state = SingleMatchState::new(pattern);
    for byte in text.as_bytes() {
        if !state.process_byte(*byte) {
            return false;
        }
    }
    state.is_done()
}

#[derive(Debug, Clone, Eq)]
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

impl Hash for MatchResult {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            MatchResult::Single(i) => i.hash(state),
            MatchResult::List(list) => {
                let mut list = list.clone();
                list.sort();
                list.hash(state);
            }
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

#[derive(Default, Copy, Clone)]
pub struct SingleMatchState<'a> {
    pattern: &'a [u8],
    offset: usize,
}

impl<'a> SingleMatchState<'a> {
    pub fn new(pattern: &'a str) -> Self {
        SingleMatchState {
            pattern: pattern.as_bytes(),
            offset: 0,
        }
    }

    fn remaining_pattern(&self) -> &'a [u8] {
        &self.pattern[self.offset..]
    }

    /// Backtrack to the latest wildcard
    fn backtrack(&mut self) -> bool {
        if self.offset == 0 {
            return false;
        }

        if let Some(new_offset) = self.pattern[0..self.offset]
            .iter()
            .copied()
            .rposition(|c| c == 0)
        {
            self.offset = new_offset;
            true
        } else {
            false
        }
    }

    pub fn process_byte(&mut self, byte: u8) -> bool {
        let pattern = self.remaining_pattern();
        match (pattern.get(0), pattern.get(1)) {
            (Some(0), Some(p)) if *p == byte => {
                self.offset += 2;
                true
            }
            (Some(0), _) => true,
            (Some(p), _) if *p == byte => {
                self.offset += 1;
                true
            }
            _ => {
                if self.backtrack() {
                    self.process_byte(byte)
                } else {
                    false
                }
            }
        }
    }

    pub fn is_done(&self) -> bool {
        matches!(self.remaining_pattern(), [] | [0])
    }
}

#[test]
fn test_matcher() {
    use crate::logline::Exception;
    use time::OffsetDateTime;

    const STATEMENTS: &[LoggingStatement] = &[
        LoggingStatement {
            line: 68,
            level: LogLevel::Exception,
            path: "foo",
            placeholders: &[],
            pattern: "Not allowed to rename a shared album",
            exception: None,
        },
        LoggingStatement {
            line: 69,
            level: LogLevel::Error,
            path: "bar",
            placeholders: &[],
            pattern: "You are not allowed to edit link shares that you don't own",
            exception: None,
        },
        LoggingStatement {
            line: 69,
            level: LogLevel::Error,
            path: "asd",
            placeholders: &["$mimeType"],
            pattern: "Unsupported query value for mimetype: \0, only values in the format \"mime/type\" or \"mime/%\" are supported",
            exception: None,
        },
        LoggingStatement {
            line: 68,
            level: LogLevel::Exception,
            path: "short",
            placeholders: &["$path"],
            pattern: "Not allowed to rename \0",
            exception: None,
        },
        LoggingStatement {
            line: 68,
            level: LogLevel::Exception,
            path: "short",
            placeholders: &["$path"],
            pattern: "Not allowed to rename \0",
            exception: Some("Bar\\FooException"),
        },
        LoggingStatement {
            line: 68,
            level: LogLevel::Error,
            path: "short",
            placeholders: &["$path"],
            pattern: "Not allowed to rename \0 to \0",
            exception: None,
        },
    ];
    let matcher = Matcher::new(&StatementList::new(vec![("", STATEMENTS)]));
    assert_eq!(
        Some(MatchResult::Single(0)),
        matcher.match_log(&LogLine {
            version: "29",
            app: "core",
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
            version: "29",
            app: "core",
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
            version: "29",
            app: "core",
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
            version: "29",
            app: "core",
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
                version: "29",
                app: "core",
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
                version: "29",
                app: "core",
                level:  LogLevel::Error,
                message: "Unsupported query value for mimetype: %/text, only values in the format \"mime/type\" or \"mime/%\" are supported".into(),
                exception: Some(Exception {
                    message: "".into(),
                    exception: "Bar\\FooException".into(),
                    file: "short".into(),
                    line: 68,
                }),
                time: OffsetDateTime::now_utc(),
                index: 0,
            }
        )
    );
    assert_eq!(
        Some(MatchResult::Single(5)),
        matcher.match_log(&LogLine {
            version: "29",
            app: "core",
            level: LogLevel::Error,
            message: "Not allowed to rename 'foo to' to to2".into(),
            exception: None,
            time: OffsetDateTime::now_utc(),
            index: 0,
        })
    );
}
