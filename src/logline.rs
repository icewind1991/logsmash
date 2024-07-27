use ahash::AHasher;
use logsmash_data::LogLevel;
use serde::Deserialize;
use std::hash::{Hash, Hasher};
use time::OffsetDateTime;
use tinystr::TinyAsciiStr;

#[derive(Deserialize)]
pub struct LogLine {
    #[serde(default)]
    pub index: usize,
    pub version: TinyAsciiStr<16>,
    pub level: LogLevel,
    pub message: String,
    pub exception: Option<Exception>,
    pub app: TinyAsciiStr<32>,
    #[serde(with = "time::serde::iso8601")]
    pub time: OffsetDateTime,
}

impl LogLine {
    pub fn identity(&self) -> u64 {
        let mut hasher = AHasher::default();
        self.message.hash(&mut hasher);
        self.level.hash(&mut hasher);
        self.exception.hash(&mut hasher);
        self.app.hash(&mut hasher);
        hasher.finish()
    }
}

impl LogLine {
    pub fn display(&self) -> String {
        if let Some(exception) = self.exception.as_ref() {
            format!(
                "{}{}{}({}) - {} line {}",
                if self.message.starts_with("Exception thrown:") {
                    ""
                } else {
                    self.message.as_str()
                },
                if self.message.starts_with("Exception thrown:") {
                    ""
                } else {
                    ": "
                },
                exception.exception,
                exception.message,
                exception.file,
                exception.line
            )
        } else {
            self.message.clone()
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Exception {
    pub message: String,
    pub exception: String,
    pub file: String,
    pub line: usize,
    pub previous: Option<Box<Exception>>,
}

impl Hash for Exception {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.message.hash(state);
        self.exception.hash(state);
        self.file.hash(state);
        self.line.hash(state);
    }
}
