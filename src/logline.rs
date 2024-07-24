use cloud_log_analyser_data::LogLevel;
use serde::Deserialize;
use tinystr::TinyAsciiStr;

#[derive(Deserialize)]
pub struct LogLine {
    pub version: TinyAsciiStr<8>,
    pub level: LogLevel,
    pub message: String,
    pub exception: Option<Exception>,
    pub app: TinyAsciiStr<16>,
}

impl LogLine {
    pub fn major_version(&self) -> Option<u32> {
        let major = self
            .version
            .split_once('.')
            .map(|(major, _)| major)
            .unwrap_or(self.version.as_str());
        major.parse().ok()
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Exception {
    pub exception: String,
    pub file: String,
    pub line: usize,
    pub previous: Option<Box<Exception>>,
}
