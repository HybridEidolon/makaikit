use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvilityInfo {
    #[serde(rename = "ID")]
    pub id: i32,

    pub learn_lv: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharaClassData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,

    pub name: String,
    pub ref_id: i32,
    pub chara_id: i32,
    pub equip_id: [i32; 4],
    pub extra_color_flag: i32,

    #[serde(rename = "itemWorldDefaultAIType")]
    pub item_world_default_ai_type: i32,

    pub evility: Vec<EvilityInfo>,
    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for CharaClassData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
