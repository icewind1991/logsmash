use crate::matcher::Matcher;
use clap::Parser;
use cloud_log_analyser_data::get_statements;
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::once;
use std::ops::AddAssign;

mod matcher;

#[derive(Debug, Parser)]
enum Args {
    Log(LogCommand),
    File(FileCommand),
}

#[derive(Debug, Parser)]
struct LogCommand {
    line: String,
}

#[derive(Debug, Parser)]
struct FileCommand {
    file: String,
}

#[derive(Deserialize)]
struct LogLine<'a> {
    version: &'a str,
    level: i64,
    message: Cow<'a, str>,
}

fn main() {
    let args = Args::parse();

    match args {
        Args::Log(LogCommand { line }) => {
            let parsed_line: LogLine = serde_json::from_str(&line).unwrap();
            let major = parsed_line.version.split(".").next().unwrap();
            let major = major.parse().unwrap();
            let statements = get_statements("server", major);
            let matcher = Matcher::new(statements);
            let index = matcher.match_log(parsed_line.level.into(), parsed_line.message.as_ref());
            if let Some(index) = index {
                let statement = &statements[index];
                println!("match found: {} line {}", statement.path, statement.line);
            } else {
                eprintln!("No match found");
            }
        }
        Args::File(FileCommand { file }) => {
            let file = BufReader::new(File::open(file).unwrap());
            let mut counts: HashMap<usize, usize> = HashMap::default();
            let mut lines = file.lines().flatten();
            let first = lines.next().unwrap();
            let first_parsed: LogLine = serde_json::from_str(&first).unwrap();

            let major = first_parsed.version.split(".").next().unwrap();
            let major = major.parse().unwrap();
            let statements = get_statements("server", major);
            let matcher = Matcher::new(statements);

            let lines = once(first).chain(lines);
            let mut error_count = 0;
            for line in lines {
                if line.starts_with('{') {
                    let parsed = match serde_json::from_str::<LogLine>(&line) {
                        Ok(parsed) => parsed,
                        Err(_) => {
                            error_count += 1;
                            continue;
                        }
                    };
                    if let Some(index) =
                        matcher.match_log(parsed.level.into(), parsed.message.as_ref())
                    {
                        counts.entry(index).or_default().add_assign(1);
                    }
                }
            }
            let mut counts: Vec<(_, _)> = counts.into_iter().collect();
            counts.sort_by_key(|(_, count)| *count);
            counts.reverse();
            for (index, count) in counts {
                let statement = &statements[index];
                println!("{} line {}: {}", statement.path, statement.line, count);
            }
            if error_count > 0 {
                eprintln!("{error_count} lines failed to parse as valid log json");
            }
        }
    }
}
