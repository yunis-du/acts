use crate::{
    Result,
    store::{LogRecord, db::mem::DbDocument},
};
use serde_json::{Value as JsonValue, json};
use std::collections::HashMap;

impl DbDocument for LogRecord {
    fn id(&self) -> &str {
        &self.id
    }

    fn doc(&self) -> Result<HashMap<String, JsonValue>> {
        let mut map = HashMap::new();
        map.insert("id".to_string(), json!(self.id.clone()));
        map.insert("tid".to_string(), json!(self.tid.clone()));
        map.insert("pid".to_string(), json!(self.pid.clone()));
        map.insert("level".to_string(), json!(self.level));
        map.insert("content".to_string(), json!(self.content.clone()));
        map.insert("timestamp".to_string(), json!(self.timestamp));
        Ok(map)
    }
}
