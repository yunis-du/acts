use crate::{LogInfo, LogRecord, Result, scheduler::Runtime, store::PageData};
use std::sync::Arc;
use tracing::instrument;

use super::ExecutorQuery;

#[derive(Clone)]
pub struct LogExecutor {
    runtime: Arc<Runtime>,
}

impl LogExecutor {
    pub fn new(rt: &Arc<Runtime>) -> Self {
        Self {
            runtime: rt.clone(),
        }
    }
    #[instrument(skip(self))]
    pub fn list(&self, q: &ExecutorQuery) -> Result<PageData<LogInfo>> {
        let query = q.into_query();
        match self.runtime.cache().store().logs().query(&query) {
            Ok(logs) => Ok(PageData {
                count: logs.count,
                page_size: logs.page_size,
                page_count: logs.page_count,
                page_num: logs.page_num,
                rows: logs.rows.iter().map(|m| m.into()).collect(),
            }),
            Err(err) => Err(err),
        }
    }

    #[instrument(skip(self))]
    pub fn get(&self, id: &str) -> Result<LogInfo> {
        let log = &self.runtime.cache().store().logs().find(id)?;
        Ok(log.into())
    }

    pub fn emit(&self, log: &LogRecord) -> Result<()> {
        self.runtime.emitter().emit_log(log);
        Ok(())
    }
}
