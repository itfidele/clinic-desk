
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use iced::{Command, Application};
use iced::{theme::Theme, Element};
use crate::database::db::establish_connection;
use crate::schema::users::dsl::*;
use crate::schema::users::{email, password};
use diesel::prelude::*;
use crate::database::models::User;

use super::pages::dashboard::dashboard_view;
use super::pages::login::login_view;
use super::types::clinic_desk::ClinicDesk;
use super::types::message::Message;

impl Application for ClinicDesk {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (ClinicDesk, Command<Message>) {
        ClinicDesk {
            is_logged_in: false,
            email: "".to_string(),
            password: "".to_string(),
            alert_msg: "".to_string(),
        };
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        if self.is_logged_in {
            String::from("Clinic Desk - Dashboard")
        } else {
            String::from("Clinic Desk - Login")
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Login => {
                self.login();
            }
            Message::Logout => {
                self.logout();
            }
            Message::EmailChanged(other_email)=>{
                self.email = other_email;
            }
            Message::PasswordChanged(pass)=>{
                self.password = pass;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        if !self.is_logged_in {
            login_view(self)
        } else {
            dashboard_view(self)
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
            .filter(email.eq(&self.email))
            .filter(password.eq(&self.password))
            .select(User::as_select())
            .first::<User>(connection)
            .optional();
    
        match result {
            Ok(Some(_user))=>{
                println!("User logged in {:?}", _user);
                self.is_logged_in = true;
                self.email = "".to_string();
                self.password = "".to_string();
            },
            Ok(None) =>{
                self.is_logged_in = false;
                self.alert_msg = "Invalid email or password".to_string();
            }
            Err(_)=>{
                self.alert_msg = "Something gone wrong, please try again later ".to_string();
            }
        }
    }
    fn logout(&mut self) {
        self.is_logged_in = false;
    }

    
   
}

