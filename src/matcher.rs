use crate::logline::{Exception, LogLine};
use itertools::Either;
use logsmash_data::{LogLevel, LoggingStatement, StatementList};
use std::hash::{Hash, Hasher};
use std::iter::once;
use std::ops::Range;

#[derive(Debug)]
pub struct LogMatch {
    level: LogLevel,
    pattern: &'static str,
    exception: Option<&'static str>,
    path: &'static str,
    line: usize,
    index: usize,
}

impl LogMatch {
    pub fn new(index: usize, statement: &LoggingStatement) -> LogMatch {
        LogMatch {
            level: statement.level,
            pattern: if statement.has_meaningful_message() {
                statement.pattern
            } else {
                ""
            },
            exception: statement.exception,
            path: statement.path,
            line: statement.line,
            index,
        }
    }

    pub fn pattern_len(&self) -> usize {
        self.pattern.len()
    }
}

pub struct Matcher {
    matches: Vec<LogMatch>,
    level_ranges: Vec<Range<usize>>,
}

impl Matcher {
    pub fn new(statements: &StatementList) -> Matcher {
        let mut matches: Vec<_> = statements
            .iter()
            .enumerate()
            .map(|(index, statement)| LogMatch::new(index, statement))
            .collect();
        matches.sort_by(|a, b| {
            // sort first by level, then by longest pattern
            a.level
                .cmp(&b.level)
                .then(a.pattern_len().cmp(&b.pattern_len()).reverse())
        });
        let level_starts =
            LogLevel::iter().map(|level| matches.iter().position(|m| m.level == level));
        let level_ends =
            LogLevel::iter().map(|level| matches.iter().rposition(|m| m.level == level));
        let level_ranges = level_starts
            .zip(level_ends)
            .map(|(start, end)| match (start, end) {
                (Some(start), Some(end)) => start..end + 1,
                _ => 0..0,
            })
            .collect();

        Matcher {
            matches,
            level_ranges,
        }
    }

    fn matches_for_level(&self, level: LogLevel) -> impl Iterator<Item = &[LogMatch]> {
        LogLevel::iter()
            .zip(self.level_ranges.iter())
            .filter_map(move |(match_level, range)| {
                level.matches(match_level).then_some(range.clone())
            })
            .map(|range| &self.matches[range])
    }

    pub fn match_log(&self, log: &LogLine) -> Option<MatchResult> {
        let mut best_match = None;
        let mut best_length = 0;

        if let Some(exception) = &log.exception {
            if let Some(index) = self.match_exception(exception) {
                return Some(MatchResult::Single(index));
            }
        }

        for matches_for_level in self.matches_for_level(log.level) {
            for log_match in matches_for_level {
                if log_match.pattern_len() < best_length {
                    break;
                }

                if !log_match.pattern.is_empty()
                    && match_single(log_match.pattern, log.message.as_ref())
                {
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
        // note that the pattern is padded with 2 \x01 bytes so we should never reach the end
        let pattern = self.remaining_pattern();
        match pattern.get(0..2) {
            Some(&[0, p]) if p == byte => {
                self.offset += 2;
                true
            }
            Some(&[0, _]) => true,
            Some(&[p, _]) if p == byte => {
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
        matches!(self.remaining_pattern(), [1, 1] | [0, 1, 1])
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
            pattern: "Not allowed to rename a shared album\x01\x01",
            exception: None,
        },
        LoggingStatement {
            line: 69,
            level: LogLevel::Error,
            path: "bar",
            placeholders: &[],
            pattern: "You are not allowed to edit link shares that you don't own\x01\x01",
            exception: None,
        },
        LoggingStatement {
            line: 69,
            level: LogLevel::Error,
            path: "asd",
            placeholders: &["$mimeType"],
            pattern: "Unsupported query value for mimetype: \0, only values in the format \"mime/type\" or \"mime/%\" are supported\x01\x01",
            exception: None,
        },
        LoggingStatement {
            line: 68,
            level: LogLevel::Exception,
            path: "short",
            placeholders: &["$path"],
            pattern: "Not allowed to rename \0\x01\x01",
            exception: None,
        },
        LoggingStatement {
            line: 68,
            level: LogLevel::Exception,
            path: "short",
            placeholders: &["$path"],
            pattern: "Not allowed to rename \0\x01\x01",
            exception: Some("Bar\\FooException"),
        },
        LoggingStatement {
            line: 68,
            level: LogLevel::Error,
            path: "short",
            placeholders: &["$path"],
            pattern: "Not allowed to rename \0 to \0\x01\x01",
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
