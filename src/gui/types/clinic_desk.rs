#[derive(Debug, Default)]
pub struct ClinicDesk {
    pub login_field: LoginField,
    pub page: Page,
    pub alert_msg: String,
}

#[derive(Debug, Default, Clone)]
pub struct LoginField {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    Login,
    Dashboard
}

impl Default for Page {
    fn default() -> Self {
        Page::Login
    }
}
