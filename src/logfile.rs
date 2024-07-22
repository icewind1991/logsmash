use crate::error::ReadError;
use itertools::Either;
use std::fs::File;
use std::io::{BufRead, BufReader};
use zip::ZipArchive;

pub enum LogFile {
    Plain(BufReader<File>),
    Zip(ZipArchive<File>),
}

impl LogFile {
    pub fn open(path: &str) -> Result<LogFile, ReadError> {
        let file = File::open(path)?;
        if path.ends_with(".zip") {
            let mut zip = ZipArchive::new(file)?;
            if zip.len() > 1 {
                return Err(ReadError::MultipleFiles);
            } else if zip.is_empty() {
                return Err(ReadError::NoFiles);
            }
            // ensure we can open the file
            let _ = zip.by_index(0)?;

            Ok(LogFile::Zip(zip))
        } else {
            Ok(LogFile::Plain(BufReader::new(file)))
        }
    }

    pub fn iter(&mut self) -> impl Iterator<Item = String> + '_ {
        match self {
            LogFile::Plain(file) => Either::Left(file.lines().flatten()),
            LogFile::Zip(zip) => {
                let file = zip.by_index(0).expect("failed to open zip content again");
                Either::Right(BufReader::new(file).lines().flatten())
            }
        }
    }
}
