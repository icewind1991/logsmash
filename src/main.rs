use crate::app::{App, LogMatch};
use crate::error::LogError;
use crate::logfile::LogFile;
use crate::logline::LogLine;
use crate::matcher::{MatchResult, Matcher};
use crate::ui::run_ui;
use base64::prelude::*;
use clap::Parser;
use logsmash_data::{default_apps, get_statements, SourceDefinition};
use main_error::MainResult;
use rayon::prelude::ParallelBridge;
use rayon::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::iter::once;

mod app;
mod error;
mod logfile;
mod logline;
mod matcher;
mod timegraph;
mod ui;

#[cfg(not(target_os = "windows"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_os = "windows"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[derive(Debug, Parser)]
struct Args {
    file: String,
    /// Collect data and exit, intended for profiling
    #[arg(long)]
    profile: bool,
}

fn main() -> MainResult {
    let args = Args::parse();

    let log_file = LogFile::open(&args.file).map_err(|err| LogError::Read {
        err,
        path: args.file,
    })?;
    let mut lines = log_file.iter();

    let mut counts: HashMap<MatchResult, Vec<usize>> = HashMap::new();
    let first = lines.next().unwrap();
    let first_parsed: LogLine = match serde_json::from_str(first) {
        Ok(first_parsed) => first_parsed,
        Err(e) => {
            eprintln!("Failed to parse the first line in the log: {:#}", e);
            return Ok(());
        }
    };

    let server_version = first_parsed.version;
    let statements = get_statements(
        once(SourceDefinition {
            name: "server",
            version: Cow::Owned(server_version.to_string()),
        })
        .chain(default_apps()),
    );
    let matcher = Matcher::new(&statements);

    let lines = once(first).chain(lines);

    let mut results: Vec<_> = lines
        .enumerate()
        .par_bridge()
        .flat_map(|(index, line)| {
            let mut parsed = serde_json::from_str::<LogLine>(line).ok()?;
            parsed.index = index;
            Some(parsed)
        })
        .map(|parsed| {
            let log_match = matcher.match_log(&parsed);
            (parsed, log_match)
        })
        .collect();

    results.sort_by_key(|(line, _)| line.index);

    let mut parsed_lines = Vec::with_capacity(1024);
    let mut unmatched_lines = Vec::with_capacity(256);

    for (parsed_index, result) in results.into_iter().enumerate() {
        let parsed = match result {
            (parsed, Some(match_result)) => {
                counts.entry(match_result).or_default().push(parsed_index);
                parsed
            }
            (parsed, None) => {
                unmatched_lines.push(parsed_index);
                parsed
            }
        };

        parsed_lines.push(parsed);
    }
    let error_count = log_file.iter().count();

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
        .into_par_iter()
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
        log_file,
    };

    if args.profile {
        return Ok(());
    }

    run_ui(app)?;

    Ok(())
}

fn copy_osc(text: &str) {
    print!("\x1B]52;c;{}\x07", BASE64_STANDARD.encode(text))
}
