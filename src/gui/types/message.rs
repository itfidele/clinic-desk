

#[derive(Debug, Clone)]
pub enum Message{
    Login,
    Logout,
    EmailChanged(String),
    PasswordChanged(String),
}