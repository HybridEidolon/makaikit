use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InnocentAffinityData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub name: String,
    pub index: i32,

    #[serde(with = "BigArray")]
    pub probability: [i32; 37],

    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for InnocentAffinityData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        ""
    }
}
