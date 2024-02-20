use crate::database::db::establish_connection;
use crate::database::models::User;
use crate::schema::users::dsl::*;
use crate::schema::users::{email, password};
use diesel::prelude::*;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use iced::{theme::Theme, Element};
use iced::{Application, Command};

use super::pages::dashboard_page::dashboard_page;
use super::pages::login_page::login_page;
use super::types::clinic_desk::{ClinicDesk, LoginField, Page};
use super::types::message::Message;

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
            Page::Dashboard => dashboard_page(self),
            Page::Logout => {
                println!("Logout");
                login_page(self)
            }
        }
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}

impl ClinicDesk {
    fn login(&mut self) {
        let connection = &mut establish_connection();
        let result = users
            .filter(email.eq(&self.login_field.email))
            .filter(password.eq(&self.login_field.password))
            .select(User::as_select())
            .first::<User>(connection)
            .optional();

        match result {
            Ok(Some(_user)) => {
                println!("User logged in {:?}", _user);
                self.page = Page::Dashboard;
                self.login_field.email = "".to_string();
                self.login_field.password = "".to_string();
            }
            Ok(None) => {
                self.page = Page::Login;
                self.alert_msg = "Invalid email or password".to_string();
            }
            Err(_) => {
                self.alert_msg = "Something gone wrong, please try again later ".to_string();
            }
        }
    }
    fn logout(&mut self) {
        self.page = Page::Login;
    }
}
