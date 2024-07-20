use crate::matcher::Matcher;
use clap::Parser;
use cloud_log_analyser_data::get_statements;
use serde::Deserialize;
use std::borrow::Cow;

mod matcher;

#[derive(Debug, Parser)]
enum Args {
    Log(LogCommand),
}

#[derive(Debug, Parser)]
struct LogCommand {
    line: String,
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
    }
}
