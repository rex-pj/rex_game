pub trait IdentityUserTrait<K> {
    fn set_id(&mut self, value: i32);
    fn set_password_hash(&mut self, value: &str);
    fn set_security_stamp(&mut self, value: &str);
    fn set_display_name(&mut self, value: &str);
    fn set_email(&mut self, value: &str);
    fn set_name(&mut self, value: &str);

    fn id(&self) -> K;
    fn password_hash(&self) -> &str;
    fn security_stamp(&self) -> &str;
    fn display_name(&self) -> Option<&str>;
    fn email(&self) -> &str;
    fn name(&self) -> &str;
}
