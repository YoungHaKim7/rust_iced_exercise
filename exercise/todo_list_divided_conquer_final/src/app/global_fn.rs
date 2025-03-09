use iced::{
    Alignment, Element, Font,
    widget::{Text, text},
};

use super::message::Message;

pub fn loading_message<'a>() -> Element<'a, Message> {
    let contents = "Loading...";
    iced::widget::center(contents).into()
}

pub fn empty_message(message: &str) -> Element<'_, Message> {
    iced::widget::center(message).into()
}

// Fonts
pub fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(Font::with_name("Iced-Todos-Icons"))
        .width(20)
        .align_x(Alignment::Center)
}

pub fn edit_icon() -> Text<'static> {
    icon('\u{F303}')
}

pub fn delete_icon() -> Text<'static> {
    icon('\u{F1F8}')
}
