use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DopingData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub ini_name: String,
    pub comment: String,
    pub name: CmlString,
    pub help: CmlString,
    pub effect: i32,
    pub r#type: i32,
    pub data1: i32,
    pub data2: i32,
    pub cond_on_flag: Vec<i32>,
    pub cond_off_flag: Vec<i32>,
    pub icon_seq: i32,
    pub effect_seq: i32,
    pub drug_inner_seq1: i32,
    pub drug_inner_seq2: i32,
    pub drug_inner_seq3: i32,
    pub tube_seq: i32,
    pub drug_seq: i32,

    #[serde(rename = "iconID")]
    pub icon_id: i32,

    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for DopingData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
