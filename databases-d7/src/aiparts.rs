use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Arg {
    pub value: i32,
    pub max: i32,
    pub min: i32,
    pub list: Vec<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiPartsData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub name: Vec<CmlString>,
    pub parts_type: i32,
    pub list_order: i32,
    pub script_func_name: String,
    pub icon_no: i32,

    #[serde(rename = "Arg1")]
    pub arg_1: Arg,

    #[serde(rename = "Arg2")]
    pub arg_2: Arg,

    pub is_end_action: i32,
    pub open_flag: i32,
    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for AiPartsData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
