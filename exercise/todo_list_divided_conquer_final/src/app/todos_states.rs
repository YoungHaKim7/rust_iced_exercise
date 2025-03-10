use super::{
    filter::Filter,
    global_fn::{empty_message, loading_message, view_controls},
    message::Message,
    save_states::SavedState,
    task_states::Task,
    taskmessage::TaskMessage,
};

use iced::{
    Alignment, Element,
    Length::Fill,
    Renderer, Subscription, Task as Command, Theme, keyboard,
    widget::{Text, TextInput, center, keyed_column, text, text_input},
    window,
};

#[derive(Debug)]
pub enum Todos {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
pub struct State {
    pub input_value: String,
    pub filter: Filter,
    pub tasks: Vec<Task>,
    pub dirty: bool,
    pub saving: bool,
}

impl Todos {
    pub const ICON_FONT: &'static [u8] = include_bytes!("../../assets/fonts/icons.ttf");

    pub fn new() -> (Self, Command<Message>) {
        (
            Self::Loading,
            Command::perform(SavedState::load(), Message::Loaded),
        )
    }

    pub fn title(&self) -> String {
        let dirty = match self {
            Todos::Loading => false,
            Todos::Loaded(state) => state.dirty,
        };
        format!("Todos{} - Iced", if dirty { "*" } else { "" })
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Todos::Loading => match message {
                Message::Loaded(Ok(state)) => {
                    *self = Todos::Loaded(State {
                        input_value: state.input_value,
                        filter: state.filter,
                        tasks: state.tasks,
                        ..State::default()
                    });
                }
                Message::Loaded(Err(_)) => {
                    *self = Todos::Loaded(State::default());
                }
                _ => {}
            },
            Todos::Loaded(state) => {
                let command = match message {
                    Message::InputChanged(value) => {
                        state.input_value = value;
                        Command::none()
                    }
                    Message::CreateTask => {
                        if !state.input_value.is_empty() {
                            state.tasks.push(Task::new(state.input_value.clone()));
                            state.input_value.clear();
                        }
                        Command::none()
                    }
                    Message::FilterChanged(filter) => {
                        state.filter = filter;
                        Command::none()
                    }
                    Message::TaskMessage(i, TaskMessage::Delete) => {
                        if i < state.tasks.len() {
                            state.tasks.remove(i);
                        }
                        Command::none()
                    }
                    Message::TaskMessage(i, task_message) => {
                        if let Some(task) = state.tasks.get_mut(i) {
                            task.update(task_message);
                        }
                        Command::none()
                    }
                    Message::ToggleFullscreen(_mode) => Command::none(),
                    _ => Command::none(),
                };
                return command;
            }
        }
        Command::none()
    }

    pub fn view(&self) -> Element<super::message::Message> {
        match self {
            Todos::Loading => loading_message(),
            Todos::Loaded(State {
                input_value,
                filter,
                tasks,
                ..
            }) => {
                let title: Text<'_, Theme, Renderer> = text("todos")
                    .width(Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .align_x(Alignment::Center);
                let input: TextInput<'_, Message> =
                    text_input("What needs to be done?", input_value)
                        .id("new-task")
                        .on_input(Message::InputChanged)
                        .on_submit(Message::CreateTask)
                        .padding(15)
                        .size(30)
                        .align_x(Alignment::Center);
                let controls = view_controls(tasks, *filter);
                let filtered_tasks = tasks.iter().filter(|task| filter.matches(task));
                let tasks: Element<_> = if filtered_tasks.count() > 0 {
                    keyed_column(
                        tasks
                            .iter()
                            .enumerate()
                            .filter(|(_, task)| filter.matches(task))
                            .map(|(i, task)| {
                                (
                                    task.id,
                                    task.view(i).map(move |msg| Message::TaskMessage(i, msg)),
                                )
                            }),
                    )
                    .spacing(10)
                    .into()
                } else {
                    empty_message(match filter {
                        Filter::All => "You have not created a task yet...",
                        Filter::Active => "All your tasks are done! :D",
                        Filter::Completed => "You have not completed a task yet...",
                    })
                };
                let content: iced::widget::Column<'_, Message> =
                    column![title, input, controls, tasks]
                        .spacing(20)
                        .max_width(800);
                center(content).into()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;
        keyboard::on_key_press(|key, modifiers| {
            let keyboard::Key::Named(key) = key else {
                return None;
            };
            match (key, modifiers) {
                (key::Named::Tab, _) => Some(Message::TabPressed {
                    shift: modifiers.shift(),
                }),
                (key::Named::ArrowUp, keyboard::Modifiers::SHIFT) => {
                    Some(Message::ToggleFullscreen(window::Mode::Fullscreen))
                }
                (key::Named::ArrowDown, keyboard::Modifiers::SHIFT) => {
                    Some(Message::ToggleFullscreen(window::Mode::Windowed))
                }
                _ => None,
            }
        })
    }
}
