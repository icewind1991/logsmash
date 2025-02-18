mod archive;

use crate::error::ReadError;
use crate::logfile::archive::{Archive, ArchiveEntry, TarArchive, ZipArchive};
use bzip2_rs::DecoderReader;
use dialoguer::Select;
use flate2::read::GzDecoder;
use ruzstd::decoding::StreamingDecoder;
use std::io::{Cursor, Read, Seek};
use xz2::read::XzDecoder;

pub struct LogFile {
    content: String,
}

impl LogFile {
    pub fn open<R: Read + Seek>(path: &str, file: R) -> Result<LogFile, ReadError> {
        if path.ends_with(".zip") {
            let mut zip = ZipArchive::new(file)?;
            return select_file(&mut zip);
        }

        if let Some(path) = path.strip_suffix(".gz") {
            let decoder = GzDecoder::new(file);
            return Self::open_no_seek(path, decoder);
        } else if let Some(path) = path.strip_suffix(".xz") {
            let decoder = XzDecoder::new(file);
            return Self::open_no_seek(path, decoder);
        } else if let Some(path) = path.strip_suffix(".bz2") {
            let decoder = DecoderReader::new(file);
            return Self::open_no_seek(path, decoder);
        } else if let Some(path) = path.strip_suffix(".zst") {
            let decoder = StreamingDecoder::new(file)?;
            return Self::open_no_seek(path, decoder);
        }

        Self::open_no_seek(path, Box::new(file))
    }

    fn open_no_seek<R: Read>(path: &str, mut file: R) -> Result<LogFile, ReadError> {
        if path.ends_with(".tar") {
            let mut zip = TarArchive::new(file)?;
            select_file(&mut zip)
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

fn select_file<A: Archive>(archive: &mut A) -> Result<LogFile, ReadError> {
    let entry = {
        let mut entries = archive
            .entries()
            .filter(|entry| !entry.name().starts_with("__MACOSX") && !entry.name().ends_with('/'))
            .collect::<Vec<_>>();

        if entries.is_empty() {
            return Err(ReadError::NoFiles);
        }

        let index = if entries.len() == 1 {
            0usize
        } else {
            let names = entries.iter().map(A::Entry::name).collect::<Vec<_>>();
            Select::new()
                .with_prompt("Select file to load?")
                .items(&names)
                .interact()
                .unwrap()
        };

        entries.remove(index)
    };
    let name = entry.name().to_string();
    let raw = entry.extract()?;

    LogFile::open(&name, Cursor::new(raw))
}
