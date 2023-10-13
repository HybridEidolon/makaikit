use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,

    #[serde(rename = "refID")]
    pub ref_id: i32,

    pub name: CmlString,
    pub comment: CmlString,
    pub script_name: String,
    pub require_flag: i32,
    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for AiData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
