use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InnocentInfo {
    #[serde(rename = "ID")]
    pub id: i32,

    pub power: i32,
    pub love: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemCustomData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub ini_name: String,
    pub comment: String,

    #[serde(rename = "refID")]
    pub ref_id: i32,

    #[serde(rename = "itemID")]
    pub item_id: i32,

    pub lv: i32,
    pub total_floor: i32,
    pub purity: i32,
    pub pop_add: i32,

    #[serde(rename = "effectIDList")]
    pub effect_id_list: [i32; 8],

    pub innocent: [InnocentInfo; 8],

    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for ItemCustomData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
