use thiserror::Error;
use zip::result::ZipError;

#[derive(Debug, Error)]
pub enum UiError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Log(#[from] LogError),
}

#[derive(Debug, Error)]
pub enum LogError {
    #[error("Error while reading input file '{path}': {err:#}")]
    Read { err: ReadError, path: String },
}

#[derive(Debug, Error)]
pub enum ReadError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Zip(#[from] ZipError),
    #[error("archive contains multiple files")]
    MultipleFiles,
    #[error("archive contains no files")]
    NoFiles,
}
