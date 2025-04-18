use crate::logfile::{LogFile, LogIndex, LogLine};
use crate::matcher::MatchResult;
use crate::timegraph::TimeGraph;
use logsmash_data::{LoggingStatementWithPathPrefix, StatementList};
use regex::{escape, Regex, RegexBuilder};
use std::cell::OnceCell;
use std::collections::BTreeMap;
use std::fmt::Display;

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
        let unmatched_line_count = if self.unmatched.count == 0 { 0 } else { 1 };
        self.matches.len() + 1 + unmatched_line_count
    }

    pub fn get_line(&self, index: LogIndex) -> Option<&'a str> {
        self.log_file.nth(index)
    }

    pub fn line_indices_by_request<'b>(
        &'b self,
        request_id: &'b str,
    ) -> impl Iterator<Item = usize> + 'b {
        self.lines
            .iter()
            .enumerate()
            .filter(move |(_, line)| line.request_id == request_id)
            .map(|(i, _)| i)
    }
}

pub struct LogMatch {
    pub result: Option<MatchResult>,
    pub count: usize,
    pub histogram: OnceCell<TimeGraph>,
    pub sparkline: OnceCell<String>,
    pub all: GroupedLines,
    pub grouped: Vec<GroupedLines>,
}

impl LogMatch {
    pub fn new(result: Option<MatchResult>, lines: Vec<usize>, all_lines: &[LogLine]) -> Self {
        let count = lines.len();
        let grouped = group_lines(all_lines, lines.iter().copied());
        let all = GroupedLines::new(lines);

        LogMatch {
            result,
            count,
            histogram: OnceCell::new(),
            sparkline: OnceCell::new(),
            grouped,
            all,
        }
    }

    pub fn sparkline(&self, app: &App) -> &str {
        self.sparkline
            .get_or_init(|| self.histogram(app).sparkline::<10>())
            .as_str()
    }

    pub fn histogram(&self, app: &App) -> &TimeGraph {
        self.histogram.get_or_init(|| {
            let min_time = app.lines[0].time;
            let max_time = app.lines.last().unwrap().time;
            let mut histogram = TimeGraph::new(min_time, max_time);
            for line in self.all.lines.iter().map(|line| &app.lines[*line]) {
                histogram.add(line.time);
            }
            histogram
        })
    }

    pub fn row_count(&self) -> usize {
        self.result.as_ref().map(|res| res.len()).unwrap_or(1)
    }

    pub fn statements<'a>(
        &'a self,
        app: &'a App,
    ) -> impl Iterator<Item = LoggingStatementWithPathPrefix> + 'a {
        self.result
            .iter()
            .flat_map(|res| res.iter())
            .filter_map(|index| app.log_statements.get(index))
    }

    pub fn matches(&self, app: &App, filter: &Filter) -> bool {
        if filter.is_empty() {
            return true;
        }
        self.statements(app).any(|statement| {
            filter.parts().all(|filter_part| {
                filter_part.is_match(statement.pattern)
                    || filter_part.is_match(statement.path)
                    || filter_part.is_match(statement.path_prefix)
                    || statement
                        .placeholders
                        .iter()
                        .any(|placeholder| filter_part.is_match(placeholder))
                    || statement
                        .exception
                        .filter(|exception| filter_part.is_match(exception))
                        .is_some()
            })
        })
    }
}

impl LogMatch {
    pub fn count(&self) -> usize {
        self.count
    }
}

fn group_lines<I: Iterator<Item = usize>>(all_lines: &[LogLine], indices: I) -> Vec<GroupedLines> {
    let mut map: BTreeMap<u64, Vec<usize>> = BTreeMap::new();

    for (i, line) in indices.map(|i| (i, &all_lines[i])) {
        map.entry(line.identity()).or_default().push(i);
    }

    let mut list: Vec<_> = map.into_values().map(GroupedLines::new).collect();
    list.sort_by_key(|list| list.len());
    list.reverse();
    list
}

pub struct GroupedLines {
    pub lines: Vec<usize>,
    pub histogram: OnceCell<TimeGraph>,
    pub sparkline: OnceCell<String>,
}

impl GroupedLines {
    pub fn new(lines: Vec<usize>) -> Self {
        GroupedLines {
            lines,
            histogram: OnceCell::new(),
            sparkline: OnceCell::new(),
        }
    }

    pub fn sparkline(&self, app: &App) -> &str {
        self.sparkline
            .get_or_init(|| self.histogram(app).sparkline::<10>())
            .as_str()
    }

    pub fn histogram(&self, app: &App) -> &TimeGraph {
        self.histogram.get_or_init(|| {
            let min_time = app.lines[0].time;
            let max_time = app.lines.last().unwrap().time;
            let mut histogram = TimeGraph::new(min_time, max_time);
            for line in self.lines.iter().map(|line| &app.lines[*line]) {
                histogram.add(line.time);
            }
            histogram
        })
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn matches(&self, app: &App, filter: &Filter) -> bool {
        if filter.is_empty() {
            return true;
        }
        let line = &app.lines[self.lines[0]];
        filter
            .parts()
            .all(|filter_part| filter_part.is_match(&line.message))
    }
}

#[derive(Default, Clone)]
pub struct Filter {
    filter: String,
    regexes: Vec<Regex>,
}

pub static EMPTY_FILTER: Filter = Filter {
    filter: String::new(),
    regexes: Vec::new(),
};

impl Filter {
    fn build_regex(filter: &str) -> Vec<Regex> {
        filter
            .split(' ')
            .map(|part| {
                RegexBuilder::new(&escape(part))
                    .case_insensitive(true)
                    .build()
                    .unwrap()
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn new(filter: String) -> Self {
        let regexes = Self::build_regex(&filter);
        Filter { filter, regexes }
    }

    pub fn parts(&self) -> impl Iterator<Item = &Regex> {
        self.regexes.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.filter.is_empty()
    }

    pub fn push(&mut self, c: char) {
        self.filter.push(c);
        self.regexes = Self::build_regex(&self.filter);
    }

    pub fn pop(&mut self) {
        self.filter.pop();
        self.regexes = Self::build_regex(&self.filter);
    }

    pub fn pop_word(&mut self) {
        let previous_word_boundary = self.filter.trim().rfind(' ').map(|i| i + 1);
        self.filter
            .truncate(previous_word_boundary.unwrap_or_default());
        self.regexes = Self::build_regex(&self.filter);
    }

    pub fn clear(&mut self) {
        self.filter.clear();
        self.regexes = Vec::new();
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.filter)
    }
}
