use gui::types::clinic_desk::ClinicDesk;
use iced::window::{self, Position};
use iced::{Application, Settings};
mod database;
mod gui;
mod schema;

fn main() -> iced::Result {
    ClinicDesk::run(Settings {
        window: window::Settings {
            size: (1190, 670), // start size
            position: Position::Centered,
            min_size: Some((1190, 610)), // min size allowed
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            icon: None,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
