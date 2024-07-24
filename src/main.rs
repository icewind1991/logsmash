use crate::app::{App, LogMatch};
use crate::error::LogError;
use crate::logfile::LogFile;
use crate::logline::LogLine;
use crate::matcher::{MatchResult, Matcher};
use crate::ui::run_ui;
use clap::Parser;
use cloud_log_analyser_data::{get_statements, MAX_VERSION};
use main_error::MainResult;
use std::collections::HashMap;
use std::iter::once;
use std::ops::AddAssign;

mod app;
mod error;
mod logfile;
mod logline;
mod matcher;
mod ui;

#[derive(Debug, Parser)]
struct Args {
    file: String,
    #[arg(long)]
    unmatched: bool,
}

fn main() -> MainResult {
    let args = Args::parse();

    let mut log_file = LogFile::open(&args.file).map_err(|err| LogError::Read {
        err,
        path: args.file,
    })?;
    let mut lines = log_file.iter();

    let mut counts: HashMap<MatchResult, usize> = HashMap::new();
    let first = lines.next().unwrap();
    let first_parsed: LogLine = serde_json::from_str(&first).unwrap();

    let statements = get_statements(first_parsed.major_version().unwrap_or(MAX_VERSION));
    let matcher = Matcher::new(&statements);

    let lines = once(first).chain(lines);
    let mut error_count = 0;
    let mut unmatched_total = 0;
    let mut unmatched_counts = HashMap::new();
    for line in lines {
        if line.starts_with('{') {
            let parsed = match serde_json::from_str::<LogLine>(&line) {
                Ok(parsed) => parsed,
                Err(_) => {
                    error_count += 1;
                    continue;
                }
            };
            if let Some(index) = matcher.match_log(&parsed) {
                counts.entry(index).or_default().add_assign(1);
            } else {
                if args.unmatched && parsed.app != "PHP" {
                    println!("{} :{:?}", parsed.message, &parsed.exception);
                }
                unmatched_total += 1;
                if let Some(entry) = unmatched_counts.get_mut(parsed.app.as_ref()) {
                    *entry += 1;
                } else {
                    unmatched_counts.insert(parsed.app.to_string(), 1);
                }
            }
        }
    }

    let mut counts: Vec<(_, _)> = counts.into_iter().collect();
    counts.sort_by_key(|(_, count)| *count);
    counts.reverse();

    let app = App {
        log_statements: statements,
        matches: counts
            .into_iter()
            .map(|(result, count)| LogMatch { result, count })
            .collect(),
    };

    run_ui(app)?;

    Ok(())
}
