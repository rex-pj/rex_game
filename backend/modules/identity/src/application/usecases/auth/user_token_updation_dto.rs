use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct UserTokenUpdationDto {
    pub updated_by_id: i32,
    pub is_actived: Option<bool>,
}
