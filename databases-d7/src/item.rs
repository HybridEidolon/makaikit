use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,

    #[serde(rename = "refID")]
    pub ref_id: i32,

    pub ini_name: String,
    pub name: CmlString,
    pub help: CmlString,
    pub rank: i32,

    #[serde(rename = "iconID")]
    pub icon_id: i32,

    pub r#type: i32,
    pub record_type: i32,

    #[serde(rename = "baseHP")]
    pub base_hp: i64,

    #[serde(rename = "baseSP")]
    pub base_sp: i64,

    pub base_param: [i64; 6],
    pub attribute_rate: [i32; 4],
    pub move_range: i32,
    pub move_type: i32,
    pub jump: i32,
    pub attack_range: i32,
    pub counter: i32,
    pub throw_range: i32,
    pub critical: i32,
    pub price: i64,

    #[serde(rename = "modelID")]
    pub model_id: i32,

    #[serde(rename = "colorID")]
    pub color_id: i32,

    #[serde(rename = "actID")]
    pub act_id: i32,

    #[serde(rename = "useItemActID")]
    pub use_item_act_id: i32,

    pub force_weapon_hand_type: i32,
    pub feature: Vec<i32>,

    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for ItemData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
