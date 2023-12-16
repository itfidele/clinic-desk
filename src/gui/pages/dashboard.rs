use iced::Element;
use iced::widget::{
    button, column, container, row, text, text_input, Button, Image, Scrollable, Space,
};

use crate::gui::types::clinic_desk::ClinicDesk;
use crate::gui::types::message::Message;

pub fn dashboard_view(desk:&ClinicDesk) -> Element<Message> {
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
    

