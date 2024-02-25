use super::pages::dashboard_page::dashboard_page;
use super::pages::login_page::login_page;
use super::types::clinic_desk::{ClinicDesk, LoginField, Page};
use super::types::message::Message;

use iced::{theme::Theme, Element};
use iced::{Application, Command};

impl Application for ClinicDesk {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (ClinicDesk, Command<Message>) {
        ClinicDesk {
            alert_msg: "".to_string(),
            login_field: LoginField {
                email: "".to_string(),
                password: "".to_string(),
            },
            page: Page::Login,
        };
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        if self.page == Page::Login {
            String::from("Clinic Desk - Dashboard")
        } else {
            String::from("Clinic Desk - Login")
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Router(page) => {
                self.page = page;
            }
            Message::EmailChanged(otheremail) => {
                self.login_field.email = otheremail;
            }
            Message::PasswordChanged(pass) => {
                self.login_field.password = pass;
            }
            Message::Aunthenticate => self.login(),
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        match self.page {
            Page::Login => login_page(self),
            Page::Dashboard => dashboard_page(self)
        }
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}
