use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Complete {
    pub r#type: i32,
    pub val1: i32,
    pub val2: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub r#type: i32,
    pub val1: i32,
    pub val2: i32,

    #[serde(rename = "identificationID")]
    pub identification_id: i32,

    pub rate: i32,
    pub is_complete: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HospitalGachaData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub comment: String,
    pub name: CmlString,
    pub help: CmlString,
    pub consume_point: i32,
    pub info: Vec<Info>,
    pub info2: Vec<Info>,
    pub complete: Complete,
    pub complete2: Complete,
    pub cond_on_flag: Vec<i32>,
    pub cond_off_flag: Vec<i32>,
    pub open_flag: i32,
    pub on_flag_when_complete: i32,

    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for HospitalGachaData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
