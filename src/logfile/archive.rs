use crate::error::ReadError;
use itertools::Either;
use std::borrow::Cow;
use std::io::{Read, Seek};
use std::iter::empty;
use std::sync::Mutex;

pub trait Archive {
    type Entry<'a>: ArchiveEntry
    where
        Self: 'a;

    fn entries(&mut self) -> impl Iterator<Item = Self::Entry<'_>>;
}

pub trait ArchiveEntry {
    fn name(&self) -> Cow<str>;

    fn extract(self) -> Result<Vec<u8>, ReadError>;
}

fn read_to_vec<R: Read>(size: usize, mut reader: R) -> Result<Vec<u8>, ReadError> {
    let mut buff = Vec::with_capacity(size.max(GB));
    reader.read_to_end(&mut buff)?;
    Ok(buff)
}

const GB: usize = 1_073_741_824;

pub struct ZipArchive<R>(Mutex<zip::ZipArchive<R>>);

impl<R: Read + Seek> ZipArchive<R> {
    pub fn new(reader: R) -> Result<Self, ReadError> {
        Ok(Self(Mutex::new(zip::ZipArchive::new(reader)?)))
    }
}

pub struct ZipEntry<'a, R> {
    id: usize,
    pub path: String,
    archive: &'a ZipArchive<R>,
}

impl<R: Read + Seek> ZipArchive<R> {
    fn extract(&self, id: usize) -> Result<Vec<u8>, ReadError> {
        let mut archive = self.0.lock().unwrap();
        let file = archive.by_index(id)?;
        read_to_vec(file.size() as usize, file)
    }
}

impl<R: Read + Seek> ArchiveEntry for ZipEntry<'_, R> {
    fn name(&self) -> Cow<str> {
        self.path.as_str().into()
    }

    fn extract(self) -> Result<Vec<u8>, ReadError> {
        self.archive.extract(self.id)
    }
}

impl<R: Read + Seek> Archive for ZipArchive<R> {
    type Entry<'a> = ZipEntry<'a, R> where R: 'a;

    fn entries(&mut self) -> impl Iterator<Item = Self::Entry<'_>> {
        let names = self
            .0
            .lock()
            .unwrap()
            .file_names()
            .map(String::from)
            .collect::<Vec<_>>();
        names.into_iter().enumerate().map(|(id, path)| Self::Entry {
            id,
            path,
            archive: self,
        })
    }
}

pub struct TarArchive<R: Read>(tar::Archive<R>);

impl<R: Read> TarArchive<R> {
    pub fn new(reader: R) -> Result<Self, ReadError> {
        Ok(Self(tar::Archive::new(reader)))
    }
}

pub struct TarEntry {
    name: String,
    content: Vec<u8>,
}

impl TarEntry {
    pub fn new<R: Read>(entry: tar::Entry<R>) -> Result<Self, ReadError> {
        // work around tar "in-order" requirement by just caching everything :(
        let name = match entry.path() {
            Ok(path) => path.display().to_string(),
            _ => "invalid path".into(),
        };
        let content = read_to_vec(entry.size() as usize, entry)?;
        Ok(TarEntry { name, content })
    }
}

impl ArchiveEntry for TarEntry {
    fn name(&self) -> Cow<str> {
        self.name.as_str().into()
    }

    fn extract(self) -> Result<Vec<u8>, ReadError> {
        Ok(self.content)
    }
}

impl<R: Read> Archive for TarArchive<R> {
    type Entry<'a> = TarEntry where R: 'a;

    fn entries(&mut self) -> impl Iterator<Item = Self::Entry<'_>> {
        match self.0.entries() {
            Ok(iter) => Either::Left(iter.flatten().flat_map(TarEntry::new)),
            _ => Either::Right(empty()),
        }
    }
}
