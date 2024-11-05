use ruzstd::frame_decoder::FrameDecoderError;
use std::string::FromUtf8Error;
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
    #[error(transparent)]
    Zstd(#[from] FrameDecoderError),
    #[error("archive contains multiple files")]
    MultipleFiles,
    #[error("archive contains no files")]
    NoFiles,
    #[error("log file contained non-utf8 characters: {0:#}")]
    Utf8(#[from] FromUtf8Error),
}
