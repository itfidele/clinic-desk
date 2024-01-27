use iced::alignment::Vertical;
use iced::widget::{column, row, text, Button, Column, Scrollable, Space, Text};
use iced::{Alignment, Element, Length, Renderer};

use crate::gui::types::clinic_desk::{ClinicDesk, Page};
use crate::gui::types::message::Message;

pub fn dashboard_page(dashboard: &ClinicDesk) -> Element<Message> {
    let btn_logout = Button::new("Logout")
        .on_press(Message::Router(Page::Logout))
        .padding(10)
        .width(iced::Length::Fill);

    let mut col_report: Column<'_, Message, Renderer> = Column::new()
        .height(Length::Fill)
        .width(Length::Fill)
        .align_items(Alignment::Center);

    col_report = col_report
        .push(Text::new("      Src IP address       Src port      Dst IP address       Dst port  Layer4   Layer7     Packets     Bytes   Country").vertical_alignment(Vertical::Center).height(Length::FillPortion(2)));

    let dashboard_btn = Button::new("Dashboard")
        .on_press(Message::Router(Page::Logout))
        .padding(10)
        .width(iced::Length::Fill);

    let patients_btn = Button::new("Patients")
        .on_press(Message::Router(Page::Logout))
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
        col_report,
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
