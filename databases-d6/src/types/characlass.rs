#[derive(Clone, Debug, serde::Deserialize)]
pub struct CharaClass {
    pub id: u32,
    pub name: String,
    pub editor_name: String,
}
