#[derive(Clone)]
pub struct CurrentUser {
    pub id: i32,
    pub email: String,
    pub roles: Vec<String>,
}
