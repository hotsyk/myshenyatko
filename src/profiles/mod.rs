pub mod storage;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::settings::SettingValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub settings: HashMap<String, SettingValue>,
}

impl Profile {
    pub fn new(name: String, settings: HashMap<String, SettingValue>) -> Self {
        Self {
            name,
            created_at: Utc::now(),
            settings,
        }
    }
}
