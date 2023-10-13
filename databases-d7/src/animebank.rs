use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimeBankData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub name: String,
    pub file_name: String,
    pub is_resident: i32,
    pub is_resident_tex: i32,
    pub is_resident_effect_tex: i32,
    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for AnimeBankData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
