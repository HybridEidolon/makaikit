use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleFlagData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub comment: String,
    pub flag_on: i32,
    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for BattleFlagData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
