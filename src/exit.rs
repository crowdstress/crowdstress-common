use crate::primitives::Section;

#[derive(Serialize, Deserialize)]
pub struct Exit {
    pub id: String,
    pub section: Section,
}
