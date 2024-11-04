mod archive;

use crate::error::ReadError;
use crate::logfile::archive::{Archive, ArchiveEntry, TarArchive, ZipArchive};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;

pub struct LogFile {
    content: String,
}

impl LogFile {
    pub fn open(path: &str) -> Result<LogFile, ReadError> {
        let file = File::open(path)?;
        if path.ends_with(".zip") {
            let mut zip = ZipArchive::new(file)?;
            let content = select_file(&mut zip)?;

            return Ok(LogFile { content });
        }

        if let Some(path) = path.strip_suffix(".gz") {
            let decoder = GzDecoder::new(file);
            return Self::open_no_seek(path, decoder);
        }

        Self::open_no_seek(path, Box::new(file))
    }

    fn open_no_seek<R: Read>(path: &str, mut file: R) -> Result<LogFile, ReadError> {
        if path.ends_with(".tar") {
            let mut zip = TarArchive::new(file)?;
            let content = select_file(&mut zip)?;

            Ok(LogFile { content })
        } else {
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            Ok(LogFile { content })
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> + Send + '_ {
        self.content.lines()
    }

    pub fn nth(&self, index: usize) -> Option<&str> {
        self.iter().nth(index)
    }
}

fn select_file<A: Archive>(archive: &mut A) -> Result<String, ReadError> {
    let entry = {
        let mut entries = archive
            .entries()
            .filter(|entry| !entry.name().starts_with("__MACOSX"))
            .collect::<Vec<_>>();

        // todo: present a picker instead
        if entries.len() > 1 {
            return Err(ReadError::MultipleFiles);
        } else if entries.is_empty() {
            return Err(ReadError::NoFiles);
        }
        entries.pop().unwrap()
    };
    let raw = entry.extract()?;
    Ok(String::from_utf8(raw)?)
}
