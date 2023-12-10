use iced::theme::Theme;
use iced::widget::image::Handle;
use iced::widget::{
    button, column, container, row, text, text_input, Button, Image, Scrollable, Space,
};
use iced::{window, Command};
use iced::{Application, Element, Settings};

mod db;
mod models;
mod schema;

fn main() -> iced::Result {
    ClinicDesk::run(Settings {
        window: window::Settings {
            size: (800, 500),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Default)]
struct ClinicDesk {
    is_logged_in: bool,
    email: String,
    password: String,
    alert_msg: String,
}

#[derive(Debug, Clone)]
enum Message {
    Login,
    Logout,
    EmailChanged(String),
    PasswordChanged(String),
}

impl Application for ClinicDesk {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

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
            Message::EmailChanged(email) => {
                self.email = email;
            }
            Message::PasswordChanged(password) => {
                self.password = password;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        if !self.is_logged_in {
            self.login_view()
        } else {
            self.dashboard_view()
        }
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}

impl ClinicDesk {
    fn login(&mut self) {
        if self.email == "admin" && self.password == "admin" {
            self.is_logged_in = true;
            self.email = "".to_string();
            self.password = "".to_string();
        } else {
            self.alert_msg = "Invalid email or password".to_string();
        }
    }

    fn logout(&mut self) {
        self.is_logged_in = false;
    }

    fn dashboard_view(&self) -> Element<Message> {
        let btn_logout = Button::new("Logout")
            .on_press(Message::Logout)
            .padding(10)
            .width(iced::Length::Fill);

        let dashboard_btn = Button::new("Dashboard")
            .on_press(Message::Login)
            .padding(10)
            .width(iced::Length::Fill);

        let patients_btn = Button::new("Patients")
            .on_press(Message::Login)
            .padding(10)
            .width(iced::Length::Fill);

        let appointments_btn = Button::new("Appointments")
            .padding(10)
            .width(iced::Length::Fill)
            .width(iced::Length::Fill);

        let listv = Scrollable::new(column!(
            text("Patient Name").width(iced::Length::Fill),
            text("Patient Name2").width(iced::Length::Fill)
        ))
        .width(iced::Length::Fill)
        .height(iced::Length::Fill);

        column!(
            text("Clinic Desk").size(20),
            row!(dashboard_btn, patients_btn, appointments_btn, btn_logout),
            Space::with_height(20),
            text("Recent Patients").size(20),
            Space::with_height(20),
            row!(
                text("Patient Name").width(iced::Length::Fill),
                text("Date").width(iced::Length::Fill),
                text("Time").width(iced::Length::Fill),
                text("Status").width(iced::Length::Fill)
            ),
            listv
        )
        .into()
    }

    fn login_view(&self) -> Element<Message> {
        let clinic_logo = Image::new(Handle::from_path("resources/clinic_logo.png"))
            .width(100)
            .height(100);

        container(
            column!(
                container(clinic_logo).width(iced::Length::Fill).center_x(),
                container(text("Clinic Patient Management System").size(18))
                    .width(iced::Length::Fill)
                    .padding(20)
                    .center_x(),
                column!(
                    text("Email"),
                    text_input("Enter your email", &self.email).on_input(Message::EmailChanged),
                    text("Password"),
                    text_input("Enter your password", &self.password)
                        .password()
                        .on_input(Message::PasswordChanged),
                    if !self.alert_msg.is_empty() {
                        text(&self.alert_msg)
                    } else {
                        text("")
                    },
                    button("Login")
                        .width(iced::Length::Fill)
                        .on_press(Message::Login)
                )
            )
            .width(400),
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
