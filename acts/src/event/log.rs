use serde::{Deserialize, Serialize};

use crate::{data, utils::shortid};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct LogRecord {
    /// message id
    pub id: String,

    /// task id
    pub tid: String,

    /// process id
    pub pid: String,

    /// log level
    pub level: String,

    /// log content
    pub content: String,

    /// log timestamp in million second
    pub timestamp: i64,
}

impl LogRecord {
    pub fn new<S: Into<String>>(tid: S, pid: S, level: S, content: S, timestamp: i64) -> Self {
        Self {
            id: shortid(),
            tid: tid.into(),
            pid: pid.into(),
            level: level.into(),
            content: content.into(),
            timestamp,
        }
    }

    pub fn is_tid(&self, tid: &str) -> bool {
        self.tid == tid
    }

    pub fn into(&self) -> data::LogRecord {
        let value = self.clone();
        data::LogRecord {
            id: value.id,
            tid: value.tid,
            pid: value.pid,
            level: value.level.into(),
            content: value.content,
            timestamp: value.timestamp,
        }
    }
}
