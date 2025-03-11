use iced::{
    Alignment::{self, Center},
    Element, Font,
    Length::Fill,
    widget::{Text, button, row, text},
};

use super::{filter::Filter, message::Message, task_states::Task};

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

pub fn view_controls(tasks: &[Task], current_filter: Filter) -> Element<Message> {
    let tasks_left = tasks.iter().filter(|task| !task.completed).count();
    let filter_button = |label, filter, current_filter| {
        let label = text(label);
        let button = button(label).style(if filter == current_filter {
            button::primary
        } else {
            button::text
        });
        button.on_press(Message::FilterChanged(filter)).padding(8)
    };
    row![
        text!(
            "{tasks_left} {} left",
            if tasks_left == 1 { "task" } else { "tasks" }
        )
        .width(Fill),
        row![
            filter_button("All", Filter::All, current_filter),
            filter_button("Active", Filter::Active, current_filter),
            filter_button("Completed", Filter::Completed, current_filter),
        ]
        .spacing(10)
    ]
    .spacing(20)
    .align_y(Center)
    .into()
}
