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
    let first_parsed: LogLine = match serde_json::from_str(&first) {
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

    let results: Vec<_> = lines
        .enumerate()
        .par_bridge()
        .filter(|(_, line)| line.starts_with('{'))
        .map(|(index, line)| {
            let mut parsed = serde_json::from_str::<LogLine>(&line)?;
            parsed.index = index;
            let log_match = matcher.match_log(&parsed);
            Result::<_, serde_json::Error>::Ok((parsed, log_match))
        })
        .collect();

    let mut error_count = 0;
    let mut parsed_lines = Vec::with_capacity(1024);
    let mut unmatched_lines = Vec::with_capacity(256);
    let mut parsed_index = 0;

    for result in results {
        let parsed = match result {
            Ok((parsed, Some(match_result))) => {
                counts.entry(match_result).or_default().push(parsed_index);
                parsed
            }
            Ok((parsed, None)) => {
                unmatched_lines.push(parsed_index);
                parsed
            }
            Err(_) => {
                error_count += 1;
                continue;
            }
        };

        parsed_lines.push(parsed);
        parsed_index += 1;
    }
    parsed_lines.sort_by_key(|line| line.index);

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
