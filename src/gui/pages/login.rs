use iced::Element;
use iced::widget::image::Handle;
use iced::widget::{
    button, column, container, row, text, text_input, Button, Image, Scrollable, Space,
};

use crate::gui::types::clinic_desk::ClinicDesk;
use crate::gui::types::message::Message;

pub fn login_view(clinic_desk:&ClinicDesk) -> Element<Message> {
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
                text_input("Enter your email", &clinic_desk.email).on_input(Message::EmailChanged),
                text("Password"),
                text_input("Enter your password", &clinic_desk.password)
                    .password()
                    .on_input(Message::PasswordChanged),
                if !clinic_desk.alert_msg.is_empty() {
                    text(&clinic_desk.alert_msg)
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

