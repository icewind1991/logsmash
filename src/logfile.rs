use crate::error::ReadError;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;

pub struct LogFile {
    content: String,
}

impl LogFile {
    pub fn open(path: &str) -> Result<LogFile, ReadError> {
        let mut file = File::open(path)?;
        if path.ends_with(".zip") {
            let mut zip = ZipArchive::new(file)?;
            let files: Vec<_> = zip
                .file_names()
                .enumerate()
                .filter(|(_, name)| !name.starts_with("__MACOSX"))
                .collect();
            if files.len() > 1 {
                return Err(ReadError::MultipleFiles);
            } else if files.is_empty() {
                return Err(ReadError::NoFiles);
            }

            let mut log = zip.by_index(files[0].0)?;
            let mut content = String::with_capacity(log.size() as usize);
            log.read_to_string(&mut content)?;

            Ok(LogFile { content })
        } else if path.ends_with(".gz") {
            let mut decoder = GzDecoder::new(file);
            let mut content = String::new();
            decoder.read_to_string(&mut content)?;

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
