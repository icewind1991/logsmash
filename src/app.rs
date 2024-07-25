use crate::logline::LogLine;
use crate::matcher::MatchResult;
use crate::timegraph::TimeGraph;
use cloud_log_analyser_data::StatementList;
use std::collections::BTreeMap;
use time::OffsetDateTime;

pub struct App {
    pub first_date: OffsetDateTime,
    pub last_date: OffsetDateTime,
    pub lines: Vec<LogLine>,
    pub log_statements: StatementList,
    pub matches: Vec<LogMatch>,
    pub error_count: usize,
    pub all: LogMatch,
    pub unmatched: LogMatch,
}

impl App {
    pub fn match_lines(&self) -> usize {
        let unmatched_line_count = if self.unmatched.lines.is_empty() {
            0
        } else {
            1
        };
        self.matches.len() + 1 + unmatched_line_count
    }
}

pub struct LogMatch {
    pub result: Option<MatchResult>,
    pub lines: Vec<usize>,
    pub histogram: TimeGraph,
    pub grouped: Vec<GroupedLines>,
}

impl LogMatch {
    pub fn new(result: Option<MatchResult>, lines: Vec<usize>, all_lines: &[LogLine]) -> Self {
        let min_time = all_lines[0].time;
        let max_time = all_lines.last().unwrap().time;
        let mut histogram = TimeGraph::new(min_time, max_time);
        for line in lines.iter().map(|line| &all_lines[*line]) {
            histogram.add(line.time);
        }
        let grouped = group_lines(all_lines, lines.iter().copied());

        LogMatch {
            result,
            lines,
            histogram,
            grouped,
        }
    }
}

impl LogMatch {
    pub fn count(&self) -> usize {
        self.lines.len()
    }
}

fn group_lines<I: Iterator<Item = usize>>(all_lines: &[LogLine], indices: I) -> Vec<GroupedLines> {
    let mut map: BTreeMap<u64, Vec<usize>> = BTreeMap::new();

    for (i, line) in indices.map(|i| (i, &all_lines[i])) {
        map.entry(line.index()).or_default().push(i);
    }

    let mut list: Vec<_> = map
        .into_values()
        .map(|lines| GroupedLines::new(lines, all_lines))
        .collect();
    list.sort_by_key(|list| list.len());
    list.reverse();
    list
}

pub struct GroupedLines {
    pub lines: Vec<usize>,
    pub histogram: TimeGraph,
}

impl GroupedLines {
    pub fn new(lines: Vec<usize>, all_lines: &[LogLine]) -> Self {
        let min_time = all_lines[0].time;
        let max_time = all_lines.last().unwrap().time;
        let mut histogram = TimeGraph::new(min_time, max_time);
        for line in lines.iter().map(|line| &all_lines[*line]) {
            histogram.add(line.time);
        }
        GroupedLines { lines, histogram }
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }
}
