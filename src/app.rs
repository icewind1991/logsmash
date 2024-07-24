use crate::logline::LogLine;
use crate::matcher::MatchResult;
use cloud_log_analyser_data::StatementList;

pub struct App {
    pub lines: Vec<LogLine>,
    pub log_statements: StatementList,
    pub matches: Vec<LogMatch>,
    pub error_count: usize,
    pub unmatched: Vec<UnMatched>,
}

pub struct LogMatch {
    pub result: MatchResult,
    pub lines: Vec<usize>,
}

impl LogMatch {
    pub fn count(&self) -> usize {
        self.lines.len()
    }
}

pub struct UnMatched {
    pub app: String,
    pub lines: Vec<usize>,
}

impl UnMatched {
    pub fn count(&self) -> usize {
        self.lines.len()
    }
}
