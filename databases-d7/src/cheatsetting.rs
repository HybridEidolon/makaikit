use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSettingData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub page: i32,
    pub enum_name: String,
    pub name: CmlString,
    pub help_text: CmlString,
    pub caption_text: CmlString,
    pub r#type: i32,
    pub int_val: i32,
    pub item_name: [CmlString; 5],
    pub open_flag: i32,
    pub order_priority: i32,
    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for CheatSettingData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
