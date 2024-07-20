use crate::error::Error;
use crate::extractor::LogExtractor;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use walkdir::WalkDir;

pub mod error;
pub mod extractor;
mod level;

pub use level::LogLevel;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LoggingStatement<'a> {
    level: LogLevel,
    path: &'a str,
    line: usize,
    message_parts: Vec<&'a str>,
}

pub fn extract_dir<W: Write>(root: &str, mut output: W) -> Result<(), Error> {
    let mut code_buff = String::with_capacity(32 * 1024 * 1024);

    writeln!(&mut output, "[").ok();

    let mut first_line = true;

    let extractor = LogExtractor::new();

    for file in WalkDir::new(root).into_iter().flatten() {
        let path = file.path();
        if let Some(path) = path.to_str() {
            if path.ends_with(".php") {
                code_buff.clear();

                let rel_path = &path[root.len()..];

                let mut fh = File::open(path).map_err(|err| Error::Open {
                    path: path.into(),
                    err,
                })?;
                fh.read_to_string(&mut code_buff)
                    .map_err(|err| Error::Read {
                        path: path.into(),
                        err,
                    })?;
                for log_item in extractor.extract(rel_path, &code_buff) {
                    if !first_line {
                        writeln!(&mut output, ",").ok();
                    }
                    first_line = false;
                    let _ = serde_json::to_writer(&mut output, &log_item);
                }
            }
        }
    }

    writeln!(&mut output, "\n]").ok();

    Ok(())
}
