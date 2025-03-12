#[derive(Default)]
pub struct FlashcardTypeUpdationDto {
    pub name: String,
    pub description: Option<String>,
    pub updated_by_id: Option<i32>,
}
