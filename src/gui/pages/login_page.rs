use crate::gui::types::clinic_desk::ClinicDesk;
use crate::gui::types::message::Message;
use iced::widget::image::Handle;
use iced::widget::{button, column, container, text, text_input, Image};
use iced::Element;

pub fn login_page(page: &ClinicDesk) -> Element<Message> {
    let clinic_logo = Image::new(Handle::from_path("resources/clinic_logo.png"))
        .width(50)
        .height(50);
    container(
        column!(
            container(clinic_logo).width(iced::Length::Fill).center_x(),
            container(text("Clinic Patient Management System").size(18))
                .width(iced::Length::Fill)
                .padding(20)
                .center_x(),
            column!(
                text("Email"),
                text_input("Enter your email", &page.login_field.email)
                    .on_input(Message::EmailChanged),
                text("Password"),
                text_input("Enter your password", &page.login_field.password)
                    .password()
                    .on_input(Message::PasswordChanged),
                match page.alert_msg.is_empty() {
                    false => text(&page.alert_msg),
                    true => text(""),
                },
                button("Login")
                    .width(iced::Length::Fill)
                    .on_press(Message::Aunthenticate)
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
