use crate::app::{App, LogMatch};
use crate::error::LogError;
use crate::logfile::LogFile;
use crate::logline::{Exception, FullException, FullLogLine, LogLine};
use crate::matcher::{MatchResult, Matcher};
use crate::ui::run_ui;
use base64::prelude::*;
use clap::Parser;
use logsmash_data::{default_apps, get_statements, SourceDefinition};
use main_error::MainResult;
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
    let lines: Vec<_> = log_file.iter().enumerate().collect();

    let mut counts: HashMap<MatchResult, Vec<usize>> = HashMap::new();
    let (_, first) = lines.first().copied().unwrap_or_default();
    let first_parsed = match parse_line(first) {
        Ok(first_parsed) => first_parsed,
        Err(e) => {
            eprintln!("Failed to parse the first line in the log: {:#}", e);
            eprintln!("{first}");
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

    let mut results: Vec<_> = lines
        .into_par_iter()
        .map(|(index, line)| {
            let mut parsed = parse_line(line);
            if let Ok(parsed) = parsed.as_mut() {
                parsed.index = index;
            };
            parsed.map_err(|err| (index, line, err))
        })
        .map(|parsed| {
            parsed.map(|parsed| {
                let log_match = matcher.match_log(&parsed);
                (parsed, log_match)
            })
        })
        .collect();

    results.sort_by_key(|res| match res {
        Ok((line, _)) => line.index,
        Err((index, _, _)) => *index,
    });

    let mut error_lines = Vec::with_capacity(32);
    let mut parsed_lines = Vec::with_capacity(1024);
    let mut unmatched_lines = Vec::with_capacity(256);

    let mut parsed_index = 0;

    for result in results.into_iter() {
        match result {
            Ok((parsed, Some(match_result))) => {
                counts.entry(match_result).or_default().push(parsed_index);
                parsed_lines.push(parsed);
                parsed_index += 1;
            }
            Ok((parsed, None)) => {
                unmatched_lines.push(parsed_index);
                parsed_lines.push(parsed);
                parsed_index += 1;
            }
            Err((_index, line, e)) => {
                error_lines.push((line.to_string(), e));
            }
        }
    }
    let error_count = log_file.iter().count() - parsed_lines.len();

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
        error_lines,
        matches,
        unmatched,
        all,
        error_count,
        log_file: &log_file,
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

fn parse_line(mut line: &str) -> Result<LogLine, serde_json::Error> {
    if let Some(pos) = line.find('{') {
        line = &line[pos..];
    }

    let mut res = serde_json::from_str::<LogLine>(line);
    if let Ok(line) = &mut res {
        if line.exception.is_none() && line.message.starts_with("{\"Exception\":") {
            if let Ok(exception) = serde_json::from_str::<Exception>(&line.message) {
                line.message = exception.message.clone();
                line.exception = Some(exception);
            }
        }
    }
    res
}

fn parse_line_full(mut line: &str) -> Result<FullLogLine, serde_json::Error> {
    if let Some(pos) = line.find('{') {
        line = &line[pos..];
    }

    let mut res = serde_json::from_str::<FullLogLine>(line);
    if let Ok(line) = &mut res {
        if line.exception.is_none() && line.message.starts_with("{\"Exception\":") {
            if let Ok(exception) = serde_json::from_str::<FullException>(&line.message) {
                line.message = exception.message.clone();
                line.exception = Some(exception);
            }
        }
    }
    res
}
