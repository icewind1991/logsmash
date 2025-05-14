use crate::app::{App, LogMatch};
use crate::error::LogError;
use crate::logfile::LogFile;
use crate::matcher::{MatchResult, Matcher};
use crate::ui::run_ui;
use base64::prelude::*;
use clap::Parser;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use logfile::logline::{Exception, FullException, FullLogLine, LogLine, CUSTOM_TIME_FORMAT};
use logsmash_data::{default_apps, get_statements, SourceDefinition};
use main_error::MainResult;
use rayon::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::iter::once;

mod app;
mod error;
mod logfile;
mod matcher;
mod timegraph;
mod ui;

#[cfg(target_env = "musl")]
use tikv_jemallocator::Jemalloc;
use time::format_description::{parse_owned, parse_strftime_owned};

#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    file: String,
    /// Collect data and exit, intended for profiling
    #[arg(long)]
    profile: bool,
    /// Date format to use when parsing log lines
    #[arg(long)]
    date_format: Option<String>,
}

fn main() -> MainResult {
    let args = Args::parse();

    if let Some(date_format) = args.date_format.as_deref() {
        let date_format = if date_format.contains('%') {
            parse_strftime_owned(date_format)
                .inspect_err(|_| eprintln!("Invalid strftime format: {date_format}"))?
        } else {
            parse_owned::<2>(date_format)
                .inspect_err(|_| eprintln!("Invalid date format: {date_format}"))?
        };

        CUSTOM_TIME_FORMAT
            .set(Some(date_format))
            .expect("Set only once");
    }

    let file = File::open(&args.file)?;
    let file = BufReader::new(file);
    let log_file = LogFile::open(&args.file, file).map_err(|err| LogError::Read {
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

    let progress_style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg} {eta}",
    )
    .expect("invalid progress bar style");

    let mut results: Vec<_> = lines
        .into_par_iter()
        .map(|(index, line)| {
            let mut parsed = parse_line(line);
            if let Ok(parsed) = parsed.as_mut() {
                parsed.index = index.into();
            };
            parsed.map_err(|err| (index, line, err))
        })
        .map(|parsed| {
            parsed.map(|parsed| {
                let log_match = matcher.match_log(&parsed);
                (parsed, log_match)
            })
        })
        .progress_with_style(progress_style)
        .collect();

    results.sort_by_key(|res| match res {
        Ok((line, _)) => line.index,
        Err((index, _, _)) => index.into(),
    });

    let mut error_lines = Vec::with_capacity(32);
    let mut parsed_lines = Vec::with_capacity(results.len());
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
