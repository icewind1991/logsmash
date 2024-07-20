use logging_extractor::error::Error;
use logging_extractor::extract_dir;
use std::env::args;
use std::fs::canonicalize;
use std::io::stdout;

fn main() -> Result<(), Error> {
    let root = args().nth(1).expect("no root provided");
    let mode = args().nth(2).unwrap_or_else(|| "json".into());
    let root = canonicalize(&root).map_err(|err| Error::RealPath {
        path: root.into(),
        err,
    })?;
    let root = root.to_str().expect("non utf8 root path");

    let stdout = stdout();

    extract_dir(root, stdout, mode == "rust")
}
