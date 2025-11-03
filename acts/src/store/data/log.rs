use std::fmt;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::store::{DbCollectionIden, StoreIden};

#[derive(Default, Debug, Copy, PartialEq, Clone, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum LogLevel {
    #[default]
    None = 0,
    Info = 1,
    Debug = 2,
    Warn = 3,
    Error = 4,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct LogRecord {
    pub id: String,
    pub tid: String,
    pub pid: String,
    pub level: LogLevel,
    pub content: String,
    pub timestamp: i64,
}

impl DbCollectionIden for LogRecord {
    fn iden() -> StoreIden {
        StoreIden::Logs
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            LogLevel::None => "none",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        })
    }
}

impl From<i8> for LogLevel {
    fn from(value: i8) -> Self {
        match value {
            1 => LogLevel::Info,
            2 => LogLevel::Debug,
            3 => LogLevel::Warn,
            4 => LogLevel::Error,
            _ => LogLevel::None,
        }
    }
}

impl From<LogLevel> for i8 {
    fn from(val: LogLevel) -> i8 {
        match val {
            LogLevel::None => 0,
            LogLevel::Info => 1,
            LogLevel::Debug => 2,
            LogLevel::Warn => 3,
            LogLevel::Error => 4,
        }
    }
}

impl From<LogLevel> for i64 {
    fn from(val: LogLevel) -> Self {
        match val {
            LogLevel::None => 0,
            LogLevel::Info => 1,
            LogLevel::Debug => 2,
            LogLevel::Warn => 3,
            LogLevel::Error => 4,
        }
    }
}

impl From<String> for LogLevel {
    fn from(val: String) -> Self {
        match val.to_lowercase().as_str() {
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => LogLevel::None,
        }
    }
}
