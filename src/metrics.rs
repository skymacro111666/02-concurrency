use dashmap::DashMap;
use std::{fmt, sync::Arc};

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Metrics {
    // define your metrics data here
    data: Arc<DashMap<String, i64>>,
}

//基本功能：增加、删除、读取
impl Metrics {
    pub fn new() -> Self {
        Metrics {
            // initialize your metrics data here
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        // println!("display counter:{:?}", counter);
        *counter += 1;
        Ok(())
    }

    // pub fn dec(&self, key: impl Into<String>) {
    //     let counter = self.data.entry(key.into()).or_insert(0);
    //     *counter -= 1;
    // }

    // pub fn snapshot(&self) -> Result<DashMap<String, i64>> {
    //     // self.data.clone()
    //     Ok(self
    //         .data
    //         .read()
    //         .map_err(|e| anyhow!(e.to_string()))?
    //         .clone())
    // }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let data = self.data.read().map_err(|_e| fmt::Error {})?;
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
