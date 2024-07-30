use ahash::AHasher;
use logsmash_data::LogLevel;
use serde::Deserialize;
use serde_json::Value;
use std::fmt::{Display, Formatter};
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
}

impl Hash for Exception {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.message.hash(state);
        self.exception.hash(state);
        self.file.hash(state);
        self.line.hash(state);
    }
}

#[derive(Deserialize, Clone)]
pub struct FullLogLine {
    #[serde(rename = "reqId")]
    pub request_id: TinyAsciiStr<32>,
    pub level: LogLevel,
    #[serde(with = "time::serde::iso8601")]
    pub time: OffsetDateTime,
    #[serde(rename = "remoteAddr")]
    pub remote_address: String,
    pub user: String,
    pub app: TinyAsciiStr<32>,
    pub method: TinyAsciiStr<16>,
    pub url: String,
    pub message: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    pub version: TinyAsciiStr<16>,
    pub exception: Option<FullException>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FullException {
    pub exception: String,
    pub message: String,
    pub code: ExceptionCode,
    pub trace: Vec<Trace>,
    pub file: String,
    pub line: usize,
    pub previous: Option<Box<FullException>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExceptionCode {
    Num(isize),
    String(String),
}

impl FullException {
    pub fn stack(&self) -> impl Iterator<Item = &FullException> + '_ {
        ExceptionStack {
            exception: Some(self),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Trace {
    #[serde(default)]
    pub file: String,
    #[serde(default)]
    pub line: usize,
    #[serde(default)]
    pub function: String,
    #[serde(default)]
    pub class: String,
    #[serde(default, rename = "type")]
    pub ty: Option<TinyAsciiStr<4>>,
    #[serde(default)]
    pub args: Vec<Value>,
}

impl Trace {
    pub fn function(&self) -> impl Display + '_ {
        TraceFunction {
            function: self.function.as_str(),
            class: self.class.as_str(),
            ty: self.ty.as_deref(),
        }
    }
}

struct TraceFunction<'a> {
    function: &'a str,
    class: &'a str,
    ty: Option<&'a str>,
}

impl Display for TraceFunction<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.ty {
            Some(ty) => write!(f, "{}{}{}", self.class, ty, self.function),
            _ => write!(f, "{}", self.function),
        }
    }
}

struct ExceptionStack<'a> {
    exception: Option<&'a FullException>,
}

impl<'a> Iterator for ExceptionStack<'a> {
    type Item = &'a FullException;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.exception?;
        self.exception = current.previous.as_deref();
        Some(current)
    }
}
