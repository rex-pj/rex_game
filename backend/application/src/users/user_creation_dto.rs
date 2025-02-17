pub struct UserCreationDto {
    pub email: String,
    pub name: String,
    pub display_name: Option<String>,
    pub password: String,
    pub security_stamp: String,
}
