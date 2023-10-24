use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemTypeData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub name: String,
    pub info_name: CmlString,
    pub is_weapon: i32,
    pub is_human_weapon: i32,
    pub is_monster_weapon: i32,
    pub is_armor: i32,
    pub is_consume: i32,

    #[serde(rename = "groupID")]
    pub group_id: i32,

    pub sale_rate: i32,
    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for ItemTypeData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
