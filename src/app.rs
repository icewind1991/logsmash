use crate::matcher::MatchResult;
use cloud_log_analyser_data::StatementList;

pub struct App {
    pub log_statements: StatementList,
    pub matches: Vec<LogMatch>,
}

pub struct LogMatch {
    pub result: MatchResult,
    pub count: usize,
}
