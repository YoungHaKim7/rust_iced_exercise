use iced::{
    Alignment::Center,
    Element,
    Length::Fill,
    widget::{button, checkbox, row, text, text_input},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    global_fn::{delete_icon, edit_icon},
    taskmessage::TaskMessage,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    description: String,
    pub completed: bool,
    #[serde(skip)]
    state: TaskState,
}

#[derive(Debug, Clone)]
pub enum TaskState {
    Idle,
    Editing,
}

impl Task {
    fn text_input_id(i: usize) -> text_input::Id {
        text_input::Id::new(format!("task-{i}"))
    }

    pub fn new(description: String) -> Self {
        Task {
            id: Uuid::new_v4(),
            description,
            completed: false,
            state: TaskState::Idle,
        }
    }

    pub fn update(&mut self, message: TaskMessage) {
        match message {
            TaskMessage::Completed(completed) => {
                self.completed = completed;
            }
            TaskMessage::Edit => {
                self.state = TaskState::Editing;
            }
            TaskMessage::DescriptionEdited(new_description) => {
                self.description = new_description;
            }
            TaskMessage::FinishEdition => {
                if !self.description.is_empty() {
                    self.state = TaskState::Idle;
                }
            }
            TaskMessage::Delete => {}
        }
    }

    pub fn view(&self, i: usize) -> Element<TaskMessage> {
        match &self.state {
            TaskState::Idle => {
                let checkbox = checkbox(&self.description, self.completed)
                    .on_toggle(TaskMessage::Completed)
                    .width(Fill)
                    .size(17)
                    .text_shaping(text::Shaping::Advanced);
                row![
                    checkbox,
                    button(edit_icon())
                        .on_press(TaskMessage::Edit)
                        .padding(10)
                        .style(button::text),
                ]
                .spacing(20)
                .align_y(Center)
                .into()
            }
            TaskState::Editing => {
                let text_input = text_input("Describe your task...", &self.description)
                    .id(Self::text_input_id(i))
                    .on_input(TaskMessage::DescriptionEdited)
                    .on_submit(TaskMessage::FinishEdition)
                    .padding(10);
                row![
                    text_input,
                    button(row![delete_icon(), "Delete"].spacing(10).align_y(Center))
                        .on_press(TaskMessage::Delete)
                        .padding(10)
                        .style(button::danger)
                ]
                .spacing(20)
                .align_y(Center)
                .into()
            }
        }
    }
}

impl Default for TaskState {
    fn default() -> Self {
        Self::Idle
    }
}
