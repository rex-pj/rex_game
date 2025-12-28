pub enum UserTokenPurposes {
    Login = 1,
    RefreshToken = 2,
    ForgotPassword = 3,
    SignupConfirmation = 4,
}

impl UserTokenPurposes {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserTokenPurposes::Login => "login",
            UserTokenPurposes::RefreshToken => "refresh_token",
            UserTokenPurposes::ForgotPassword => "forgot_password",
            UserTokenPurposes::SignupConfirmation => "signup_confirmation",
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_owned()
    }
}
