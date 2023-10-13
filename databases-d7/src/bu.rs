use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub script_name: String,

    #[serde(rename = "animeID")]
    pub anime_id: i32,

    pub seq_no: i32,
    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for BuData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        ""
    }
}
