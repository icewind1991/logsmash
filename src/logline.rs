use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize)]
pub struct LogLine<'a> {
    pub version: &'a str,
    pub level: i64,
    pub message: Cow<'a, str>,
}

impl LogLine<'_> {
    pub fn major_version(&self) -> Option<u32> {
        let major = self
            .version
            .split_once('.')
            .map(|(major, _)| major)
            .unwrap_or(self.version);
        major.parse().ok()
    }
}
