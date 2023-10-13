use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StageInfo {
    #[serde(rename = "ID")]
    pub id: i32,
    pub clear_count: i32,
    pub is_first_time_configuration: i32,
    pub cond_on_flag: Vec<i32>,
    pub cond_off_flag: Vec<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DungeonData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub name: String,

    #[serde(rename = "areaID")]
    pub area_id: i32,

    pub difficulty: i32,
    pub cond_on_flag: Vec<i32>,
    pub cond_off_flag: Vec<i32>,
    pub stage: Vec<StageInfo>,
    pub tag: i32,

    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for DungeonData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
