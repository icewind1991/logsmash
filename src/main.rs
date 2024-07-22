use crate::error::LogError;
use crate::logfile::LogFile;
use crate::logline::LogLine;
use crate::matcher::Matcher;
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
}

fn main() -> MainResult {
    let args = Args::parse();

    let mut log_file = LogFile::open(&args.file).map_err(|err| LogError::Read {
        err,
        path: args.file,
    })?;
    let mut lines = log_file.iter();

    let mut counts: HashMap<usize, usize> = HashMap::new();
    let first = lines.next().unwrap();
    let first_parsed: LogLine = serde_json::from_str(&first).unwrap();

    let statements = get_statements(
        "server",
        first_parsed.major_version().unwrap_or(MAX_VERSION),
    );
    let matcher = Matcher::new(statements);

    let lines = once(first).chain(lines);
    let mut error_count = 0;
    let mut unmatched = 0;
    for line in lines {
        if line.starts_with('{') {
            let parsed = match serde_json::from_str::<LogLine>(&line) {
                Ok(parsed) => parsed,
                Err(_) => {
                    error_count += 1;
                    continue;
                }
            };
            if let Some(index) = matcher.match_log(parsed.level.into(), parsed.message.as_ref()) {
                counts.entry(index).or_default().add_assign(1);
            } else {
                unmatched += 1;
            }
        }
    }
    let mut counts: Vec<(_, _)> = counts.into_iter().collect();
    counts.sort_by_key(|(_, count)| *count);
    counts.reverse();
    for (index, count) in counts {
        let statement = &statements[index];
        println!(
            "{}: {} line {}: {}",
            statement.message(),
            statement.path,
            statement.line,
            count
        );
    }
    if unmatched > 0 {
        eprintln!("{unmatched} lines couldn't be matched");
    }
    if error_count > 0 {
        eprintln!("{error_count} lines failed to parse as valid log json");
    }

    Ok(())
}
