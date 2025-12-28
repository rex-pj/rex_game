pub struct FlashcardFileDto {
    pub id: i32,
    pub name: Option<String>,
    pub file_name: String,
    pub content_type: String,
    pub data: Vec<u8>,
}
