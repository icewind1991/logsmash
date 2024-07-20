use clap::Parser;
use logging_extractor::error::Error;
use logging_extractor::extract_dir;
use std::fs::canonicalize;
use std::io::stdout;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    root: PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let root = canonicalize(&args.root).map_err(|err| Error::RealPath {
        path: args.root,
        err,
    })?;
    let root = root.to_str().expect("non utf8 root path");

    let stdout = stdout();

    extract_dir(root, stdout)
}
