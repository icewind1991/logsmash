use crate::error::LogError;
use crate::logfile::LogFile;
use crate::logline::LogLine;
use crate::matcher::{MatchResult, Matcher};
use clap::Parser;
use cloud_log_analyser_data::{get_statements, MAX_VERSION};
use main_error::MainResult;
use std::collections::HashMap;
use std::iter::once;
use std::ops::AddAssign;

mod error;
mod logfile;
mod logline;
mod matcher;

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

    if args.unmatched {
        return Ok(());
    }

    let mut counts: Vec<(_, _)> = counts.into_iter().collect();
    counts.sort_by_key(|(_, count)| *count);
    counts.reverse();
    for (match_result, count) in counts {
        println!("{}: {}", match_result.display(&statements), count);
    }
    if unmatched_total > 0 {
        eprintln!("\n{unmatched_total} lines couldn't be matched:");
        for (app, count) in unmatched_counts {
            eprintln!("\t{app}: {count}");
        }
    }
    if error_count > 0 {
        eprintln!("\n{error_count} lines failed to parse as valid log json");
    }

    Ok(())
}
