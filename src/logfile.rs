use crate::error::ReadError;
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
            if zip.len() > 1 {
                return Err(ReadError::MultipleFiles);
            } else if zip.is_empty() {
                return Err(ReadError::NoFiles);
            }

            let mut log = zip.by_index(0)?;
            let mut content = String::with_capacity(log.size() as usize);
            log.read_to_string(&mut content)?;

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
