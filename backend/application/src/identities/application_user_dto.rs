use chrono::{DateTime, Utc};

use crate::identities::identity_user_trait::IdentityUserTrait;

impl IdentityUserTrait<i32> for ApplicationUserDto {
    fn id(&self) -> i32 {
        self.id
    }

    fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn password_hash(&self) -> &str {
        &self.password_hash
    }

    fn security_stamp(&self) -> &str {
        &self.security_stamp
    }

    fn set_id(&mut self, value: i32) {
        self.id = value;
    }

    fn set_password_hash(&mut self, value: &str) {
        self.password_hash = String::from(value);
    }

    fn set_security_stamp(&mut self, value: &str) {
        self.security_stamp = String::from(value);
    }

    fn set_display_name(&mut self, value: &str) {
        self.display_name = Option::from(String::from(value));
    }

    fn set_email(&mut self, value: &str) {
        self.email = String::from(value);
    }

    fn set_name(&mut self, value: &str) {
        self.name = String::from(value);
    }
}

#[derive(Default)]
pub struct ApplicationUserDto {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub display_name: Option<String>,
    pub password_hash: String,
    pub security_stamp: String,
    pub created_by_id: Option<i32>,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub updated_by_id: Option<i32>,
    pub status_id: i32,
}
