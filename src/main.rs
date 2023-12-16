use gui::types::clinic_desk::ClinicDesk;
use iced::window;
use iced::{Application, Settings};
mod gui;
mod database;
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





