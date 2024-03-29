use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BgmData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,

    pub file_name: String,
    pub folder_name: String,
    pub volume: i32,
    pub loop_start: i32,
    pub loop_end: i32,
    pub sample_rate: i32,
    pub name: CmlString,
}

impl DatabaseRecord for BgmData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
