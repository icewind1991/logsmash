use crate::logfile::LogFile;
use crate::logline::LogLine;
use crate::matcher::MatchResult;
use crate::timegraph::TimeGraph;
use logsmash_data::StatementList;
use std::collections::BTreeMap;

pub struct App<'a> {
    pub lines: Vec<LogLine<'a>>,
    pub log_statements: StatementList,
    pub matches: Vec<LogMatch>,
    pub error_count: usize,
    pub all: LogMatch,
    pub unmatched: LogMatch,
    pub log_file: &'a LogFile,
    pub error_lines: Vec<(String, serde_json::Error)>,
}

impl<'a> App<'a> {
    pub fn match_lines(&self) -> usize {
        let unmatched_line_count = if self.unmatched.lines.is_empty() {
            0
        } else {
            1
        };
        self.matches.len() + 1 + unmatched_line_count
    }

    pub fn get_line(&self, index: usize) -> Option<&'a str> {
        self.log_file.nth(index)
    }
}

pub struct LogMatch {
    pub result: Option<MatchResult>,
    pub lines: Vec<usize>,
    pub histogram: TimeGraph,
    pub sparkline: String,
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
        let sparkline = histogram.sparkline::<10>();

        LogMatch {
            result,
            lines,
            histogram,
            sparkline,
            grouped,
        }
    }

    pub fn row_count(&self) -> usize {
        self.result.as_ref().map(|res| res.len()).unwrap_or(1)
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
        map.entry(line.identity()).or_default().push(i);
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
    pub sparkline: String,
}

impl GroupedLines {
    pub fn new(lines: Vec<usize>, all_lines: &[LogLine]) -> Self {
        let min_time = all_lines[0].time;
        let max_time = all_lines.last().unwrap().time;
        let mut histogram = TimeGraph::new(min_time, max_time);
        for line in lines.iter().map(|line| &all_lines[*line]) {
            histogram.add(line.time);
        }
        let sparkline = histogram.sparkline::<10>();
        GroupedLines {
            lines,
            histogram,
            sparkline,
        }
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }
}
