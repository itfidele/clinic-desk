use crate::gui::types::clinic_desk::Page;

#[derive(Debug, Clone)]
pub enum Message {
    Router(Page),
    EmailChanged(String),
    PasswordChanged(String),
    Aunthenticate,
}
