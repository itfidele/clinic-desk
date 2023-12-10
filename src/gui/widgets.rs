use iced::widget::{Button, Container};

pub fn link_btn(text: String) -> Container {
    Button::new(text).padding(10)
}
