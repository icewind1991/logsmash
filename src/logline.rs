use cloud_log_analyser_data::LogLevel;
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize)]
pub struct LogLine<'a> {
    pub version: &'a str,
    pub level: LogLevel,
    pub message: Cow<'a, str>,
    pub exception: Option<Exception<'a>>,
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

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Exception<'a> {
    pub exception: Cow<'a, str>,
    pub file: Cow<'a, str>,
    pub line: usize,
    pub previous: Option<Box<Exception<'a>>>,
}
