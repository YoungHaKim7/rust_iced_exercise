use iced::window;

use crate::error::{loaderror::LoadError, saveerror::SaveError};

use super::{filter::Filter, save_states::SavedState, taskmessage::TaskMessage};

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    CreateTask,
    FilterChanged(Filter),
    TaskMessage(usize, TaskMessage),
    TabPressed { shift: bool },
    ToggleFullscreen(window::Mode),
}
