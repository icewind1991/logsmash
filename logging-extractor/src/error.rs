use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to determine absolute root path ({}: {err:#}", path.display())]
    RealPath { path: PathBuf, err: std::io::Error },
    #[error("Failed to open source file ({}: {err:#}", path.display())]
    Open { path: PathBuf, err: std::io::Error },
    #[error("Failed to read source file ({}: {err:#}", path.display())]
    Read { path: PathBuf, err: std::io::Error },
}
