use crate::logline::LogLine;
use crate::matcher::MatchResult;
use crate::timegraph::TimeGraph;
use cloud_log_analyser_data::StatementList;

pub struct App {
    pub lines: Vec<LogLine>,
    pub histogram: TimeGraph,
    pub log_statements: StatementList,
    pub matches: Vec<LogMatch>,
    pub error_count: usize,
    pub unmatched: Vec<usize>,
    pub unmatched_histogram: TimeGraph,
}

impl App {
    pub fn match_lines(&self) -> usize {
        let unmatched_line_count = if self.unmatched.is_empty() { 0 } else { 1 };
        self.matches.len() + 1 + unmatched_line_count
    }
}

pub struct LogMatch {
    pub result: MatchResult,
    pub lines: Vec<usize>,
    pub histogram: TimeGraph,
}

impl LogMatch {
    pub fn new(result: MatchResult, lines: Vec<usize>, all_lines: &[LogLine]) -> Self {
        let min_time = all_lines[0].time;
        let max_time = all_lines.last().unwrap().time;
        let mut histogram = TimeGraph::new(min_time, max_time);
        for line in lines.iter().map(|line| &all_lines[*line]) {
            histogram.add(line.time);
        }

        LogMatch {
            result,
            lines,
            histogram,
        }
    }
}

impl LogMatch {
    pub fn count(&self) -> usize {
        self.lines.len()
    }
}
