#[derive(Clone, Debug, serde::Deserialize)]
pub struct GameFlag {
    pub id: u32,
    pub name: String,
    pub editor_desc: String,
}
