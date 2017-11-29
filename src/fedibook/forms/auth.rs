#[derive(Debug, Clone, PartialEq, FromForm)]
pub(crate) struct SignUpForm {
    pub csrf_token: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug, Clone, PartialEq, FromForm)]
pub(crate) struct SignInForm {
    csrf_token: String,
    email: String,
    password: String,
}


