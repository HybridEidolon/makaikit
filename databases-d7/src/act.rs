use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    r#type: i32,
    value: Vec<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,

    #[serde(rename = "refID")]
    pub ref_id: i32,

    pub name: CmlString,
    pub help_text: CmlString,

    #[serde(rename = "iconID")]
    pub icon_id: i32,

    pub is_use: i32,
    pub r#type: i32,
    pub record_type: i32,
    pub weapon_type: i32,
    pub attack_type: i32,
    pub target_type: i32,
    pub attribute: i32,
    pub depend: i32,
    pub range: i32,
    pub range_type: i32,
    pub act_map: i32,
    pub valid_target: i32,
    pub valid_feature: Vec<i32>,
    pub invalid_feature: Vec<i32>,
    pub upper_range: i32,
    pub lower_range: i32,
    pub is_self_target: i32,
    pub act_feature: Vec<i32>,

    #[serde(rename = "consumeSP")]
    pub consume_sp: i32,

    pub is_enhance: i32,
    pub mana: [i32; 9],
    pub power: i32,
    pub power_coefficient: [i32; 9],
    pub effect: [Effect; 5],
    pub is_enemy_disabled: i32,
    pub is_exclusive: i32,
    pub script_file_name: String,
    pub script_func_name: String,

    #[serde(rename = "scriptFileName_omit")]
    pub script_file_name_omit: String,

    #[serde(rename = "scriptFuncName_omit")]
    pub script_func_name_omit: String,

    #[serde(rename = "scriptFileName_veryomit")]
    pub script_file_name_veryomit: String,

    #[serde(rename = "scriptFuncName_veryomit")]
    pub script_func_name_veryomit: String,

    pub required_flag: Vec<i32>,

    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for ActData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
