use crate::app::Filter;
use ahash::AHasher;
use logsmash_data::LogLevel;
use serde::Deserialize;
use serde_json::Value;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use time::format_description::well_known::iso8601::{Config, EncodedConfig, TimePrecision};
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;
use tinystr::TinyAsciiStr;

pub const TIME_FORMAT: EncodedConfig = Config::DEFAULT
    .set_time_precision(TimePrecision::Second {
        decimal_digits: None,
    })
    .encode();

#[derive(Deserialize)]
pub struct LogLine<'a> {
    #[serde(default)]
    pub index: usize,
    #[serde(rename = "reqId")]
    pub request_id: TinyAsciiStr<32>,
    pub version: &'a str,
    pub level: LogLevel,
    pub message: Cow<'a, str>,
    pub exception: Option<Exception<'a>>,
    pub app: Cow<'a, str>,
    #[serde(with = "date")]
    pub time: OffsetDateTime,
}

mod date {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer};
    use time::format_description::well_known::Iso8601;
    use time::format_description::well_known::Rfc2822;
    use time::format_description::well_known::Rfc3339;
    use time::format_description::BorrowedFormatItem;
    use time::macros::format_description;
    use time::parsing::Parsable;
    use time::{OffsetDateTime, PrimitiveDateTime};

    const FORMATS: &[&[BorrowedFormatItem]] = &[
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
        format_description!(
            "[month repr:long case_sensitive:false] [day], [year] [hour]:[minute]:[second]"
        ),
    ];

    fn try_format(str: &str, format: &(impl Parsable + ?Sized)) -> Option<OffsetDateTime> {
        if let Ok(date) = OffsetDateTime::parse(str, format) {
            Some(date)
        } else if let Ok(date) = PrimitiveDateTime::parse(str, format) {
            Some(date.assume_utc())
        } else {
            None
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<OffsetDateTime, D::Error> {
        let str = <&str>::deserialize(deserializer)?;

        if let Some(date) = try_format(str, &Iso8601::DATE_TIME_OFFSET) {
            return Ok(date);
        }
        if let Some(date) = try_format(str, &Iso8601::DATE_TIME) {
            return Ok(date);
        }
        if let Some(date) = try_format(str, &Rfc3339) {
            return Ok(date);
        }
        if let Some(date) = try_format(str, &Rfc2822) {
            return Ok(date);
        }

        for format in FORMATS {
            if let Some(date) = try_format(str, format) {
                return Ok(date);
            }
        }
        Err(D::Error::custom(format_args!(
            "Failed to parse date: {}",
            str
        )))
    }
}

pub fn format_time(time: OffsetDateTime) -> String {
    time.format(&Iso8601::<TIME_FORMAT>)
        .unwrap_or_else(|_| "Invalid time".into())
}

impl<'a> LogLine<'a> {
    pub fn identity(&self) -> u64 {
        let mut hasher = AHasher::default();
        self.message.hash(&mut hasher);
        self.level.hash(&mut hasher);
        self.exception.hash(&mut hasher);
        self.app.hash(&mut hasher);
        hasher.finish()
    }

    pub fn display(&'a self) -> Cow<'a, str> {
        if let Some(exception) = self.exception.as_ref() {
            Cow::Owned(format!(
                "{}{}{}({}) - {} line {}",
                if self.message.starts_with("Exception thrown:") {
                    ""
                } else {
                    &self.message
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
            ))
        } else {
            Cow::Borrowed(&self.message)
        }
    }

    pub fn matches(&self, filter: &Filter) -> bool {
        if filter.is_empty() {
            return true;
        }
        // todo: exception, more?
        filter.matches(&self.app)
            || filter.matches(&self.message)
            || filter.matches(self.request_id.as_str())
    }
}

#[derive(Deserialize, Debug, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct Exception<'a> {
    pub message: Cow<'a, str>,
    pub exception: Cow<'a, str>,
    pub file: Cow<'a, str>,
    pub line: usize,
}

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct FullLogLine {
    #[serde(rename = "reqId")]
    pub request_id: TinyAsciiStr<32>,
    pub level: LogLevel,
    #[serde(with = "date")]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
