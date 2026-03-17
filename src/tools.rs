use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetCurrentTime;

impl GetCurrentTime {
    pub fn execute(&self) -> String {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
