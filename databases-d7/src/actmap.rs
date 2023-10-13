use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MapLine {
    pub check: i32,
    pub column: [i32; 21],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActMapData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub name: String,
    pub range_type: i32,
    pub rot_type: i32,
    pub line: Vec<MapLine>,
}

impl DatabaseRecord for ActMapData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
