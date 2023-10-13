use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub name: CmlString,
    pub description: Vec<CmlString>,
    pub icon_seq: i32,
    pub bg_seq: [i32; 3],
    pub item_world_material: i32,

    #[serde(rename = "texID")]
    pub tex_id: i32,

    pub is_extra_map: i32,

    #[serde(rename = "stageID")]
    pub stage_id: i32,

    #[serde(rename = "stageID_Shura")]
    pub stage_id_shura: i32,

    pub use_flag: i32,
    pub stage_type: Vec<i32>,
    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for AreaData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
