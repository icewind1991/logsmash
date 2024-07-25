use crate::app::{App, LogMatch};
use crate::error::LogError;
use crate::logfile::LogFile;
use crate::logline::LogLine;
use crate::matcher::{MatchResult, Matcher};
use crate::ui::run_ui;
use clap::Parser;
use logsmash_data::{get_statements, MAX_VERSION};
use main_error::MainResult;
use std::collections::HashMap;
use std::iter::once;

mod app;
mod error;
mod logfile;
mod logline;
mod matcher;
mod timegraph;
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

    let mut counts: HashMap<MatchResult, Vec<usize>> = HashMap::new();
    let first = lines.next().unwrap();
    let first_parsed: LogLine = match serde_json::from_str(&first) {
        Ok(first_parsed) => first_parsed,
        Err(e) => {
            eprintln!("Failed to parse the first line in the log: {:#}", e);
            return Ok(());
        }
    };

    let statements = get_statements(first_parsed.major_version().unwrap_or(MAX_VERSION));
    let matcher = Matcher::new(&statements);

    let lines = once(first).chain(lines);
    let mut error_count = 0;
    let mut unmatched_counts: HashMap<String, Vec<usize>> = HashMap::new();
    let mut parsed_lines = Vec::with_capacity(1024);
    let mut unmatched_lines = Vec::with_capacity(256);
    let mut i = 0;
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
                counts.entry(index).or_default().push(i);
            } else {
                if args.unmatched && parsed.app != "PHP" {
                    println!("{} :{:?}", parsed.message, &parsed.exception);
                }
                if let Some(entry) = unmatched_counts.get_mut(parsed.app.as_str()) {
                    entry.push(i)
                } else {
                    unmatched_lines.push(i);
                }
            }
            parsed_lines.push(parsed);
            i += 1;
        }
    }

    let mut matched_lines: Vec<(_, _)> = counts.into_iter().collect();
    matched_lines.sort_by_key(|(_, lines)| lines.len());
    matched_lines.reverse();

    let all = LogMatch::new(
        None,
        parsed_lines.iter().enumerate().map(|(i, _)| i).collect(),
        &parsed_lines,
    );
    let unmatched = LogMatch::new(None, unmatched_lines, &parsed_lines);

    let matches = matched_lines
        .into_iter()
        .map(|(result, lines)| LogMatch::new(Some(result), lines, &parsed_lines))
        .collect();

    let app = App {
        first_date: parsed_lines[0].time,
        last_date: parsed_lines.last().unwrap().time,
        lines: parsed_lines,
        log_statements: statements,
        matches,
        unmatched,
        all,
        error_count,
    };

    run_ui(app)?;

    Ok(())
}
