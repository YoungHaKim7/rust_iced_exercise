use async_std::fs::{self, File};
use async_std::io::ReadExt;
use iced::Alignment;
use iced::futures::AsyncWriteExt;
use iced::keyboard;
use iced::widget::{Text, button, center, checkbox, column, keyed_column, row, text, text_input};
use iced::window;
use iced::window::Mode;
use iced::{Center, Element, Fill, Font, Subscription, Task as Command};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing_subscriber;
use uuid::Uuid;

#[derive(Debug)]
enum Todos {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
struct State {
    input_value: String,
    filter: Filter,
    tasks: Vec<Task>,
    dirty: bool,
    saving: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    CreateTask,
    FilterChanged(Filter),
    TaskMessage(usize, TaskMessage),
    TabPressed { shift: bool },
    ToggleFullscreen(window::Mode),
}

impl Todos {
    const ICON_FONT: &'static [u8] = include_bytes!("../fonts/icons.ttf");

    fn new() -> (Self, Command<Message>) {
        (
            Self::Loading,
            Command::perform(SavedState::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        let dirty = match self {
            Todos::Loading => false,
            Todos::Loaded(state) => state.dirty,
        };
        format!("Todos{} - Iced", if dirty { "*" } else { "" })
    }

    fn update(&mut self, message: Message) -> Command<Message> {
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
                    Message::ToggleFullscreen(mode) => {
                        // if let Some(window) = window::get_latest().0 {
                        // if let Some((window, _)) = window::get_latest() {
                        //     if mode == Mode::Fullscreen {
                        //         window.set_fullscreen(true);
                        //     } else {
                        //         window.set_fullscreen(false);
                        //     }
                        // }
                        // if let Some(window) = window::get_latest().0 {
                        //     if mode == Mode::Fullscreen {
                        //         window.set_fullscreen(true);
                        //     } else {
                        //         window.set_fullscreen(false);
                        //     }
                        // }
                        Command::none()
                    }
                    _ => Command::none(),
                };
                return command;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        match self {
            Todos::Loading => loading_message(),
            Todos::Loaded(State {
                input_value,
                filter,
                tasks,
                ..
            }) => {
                let title = text("todos")
                    .width(Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .align_x(Alignment::Center);
                let input = text_input("What needs to be done?", input_value)
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
                let content = column![title, input, controls, tasks]
                    .spacing(20)
                    .max_width(800);
                center(content).into()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    description: String,
    completed: bool,
    #[serde(skip)]
    state: TaskState,
}

#[derive(Debug, Clone)]
pub enum TaskState {
    Idle,
    Editing,
}

impl Default for TaskState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    Completed(bool),
    Edit,
    DescriptionEdited(String),
    FinishEdition,
    Delete,
}

impl Task {
    fn text_input_id(i: usize) -> text_input::Id {
        text_input::Id::new(format!("task-{i}"))
    }

    fn new(description: String) -> Self {
        Task {
            id: Uuid::new_v4(),
            description,
            completed: false,
            state: TaskState::Idle,
        }
    }

    fn update(&mut self, message: TaskMessage) {
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

    fn view(&self, i: usize) -> Element<TaskMessage> {
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

fn view_controls(tasks: &[Task], current_filter: Filter) -> Element<Message> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}

impl Filter {
    fn matches(self, task: &Task) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !task.completed,
            Filter::Completed => task.completed,
        }
    }
}

fn loading_message<'a>() -> Element<'a, Message> {
    let contents = "Loading...";
    iced::widget::center(contents).into()
}

fn empty_message(message: &str) -> Element<'_, Message> {
    iced::widget::center(message).into()
}

// Fonts
fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(Font::with_name("Iced-Todos-Icons"))
        .width(20)
        .align_x(Alignment::Center)
}

fn edit_icon() -> Text<'static> {
    icon('\u{F303}')
}

fn delete_icon() -> Text<'static> {
    icon('\u{F1F8}')
}

// Persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    input_value: String,
    filter: Filter,
    tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
enum LoadError {
    File,
    Format,
}

#[derive(Debug, Clone)]
enum SaveError {
    File,
    Write,
    Format,
}

#[cfg(not(target_arch = "wasm32"))]
impl SavedState {
    fn path() -> PathBuf {
        let mut path =
            if let Some(project_dirs) = directories::ProjectDirs::from("rs", "Iced", "Todos") {
                project_dirs.data_dir().into()
            } else {
                std::env::current_dir().unwrap_or_default()
            };
        path.push("todos.json");
        path
    }

    async fn load() -> Result<SavedState, LoadError> {
        let mut contents = String::new();
        let mut file = File::open(Self::path())
            .await
            .map_err(|_| LoadError::File)?;
        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::File)?;
        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }

    async fn save(self) -> Result<(), SaveError> {
        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::Format)?;
        let path = Self::path();
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).await.map_err(|_| SaveError::File)?;
        }
        {
            let mut file = File::create(path).await.map_err(|_| SaveError::File)?;
            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }
        // This is a simple way to save at most once every couple seconds
        async_std::task::sleep(std::time::Duration::from_secs(2)).await;
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl SavedState {
    fn storage() -> Option<web_sys::Storage> {
        let window = web_sys::window()?;
        window.local_storage().ok()?
    }

    async fn load() -> Result<SavedState, LoadError> {
        let storage = Self::storage().ok_or(LoadError::File)?;
        let contents = storage
            .get_item("state")
            .map_err(|_| LoadError::File)?
            .ok_or(LoadError::File)?;
        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }

    async fn save(self) -> Result<(), SaveError> {
        let storage = Self::storage().ok_or(SaveError::File)?;
        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::Format)?;
        storage
            .set_item("state", &json)
            .map_err(|_| SaveError::Write)?;
        wasmtimer::tokio::sleep(std::time::Duration::from_secs(2)).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::{Settings, Theme};
    use iced_test::selector::text;
    use iced_test::{Error, Simulator};

    fn simulator(todos: &Todos) -> Simulator<Message> {
        Simulator::with_settings(
            Settings {
                fonts: vec![Todos::ICON_FONT.into()],
                ..Settings::default()
            },
            todos.view(),
        )
    }

    #[test]
    fn it_creates_a_new_task() -> Result<(), Error> {
        let (mut todos, _command) = Todos::new();
        let _command = todos.update(Message::Loaded(Err(LoadError::File)));
        let mut ui = simulator(&todos);
        let _input = ui.click("new-task")?;
        let _ = ui.typewrite("Create the universe");
        let _ = ui.tap_key(keyboard::key::Named::Enter);
        for message in ui.into_messages() {
            let _command = todos.update(message);
        }
        let mut ui = simulator(&todos);
        let _ = ui.find(text("Create the universe"))?;
        let snapshot = ui.snapshot(&Theme::Dark)?;
        assert!(
            snapshot.matches_hash("snapshots/creates_a_new_task")?,
            "snapshots should match!"
        );
        Ok(())
    }
}

fn main() -> iced::Result {
    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();
    iced::application(Todos::title, Todos::update, Todos::view)
        .subscription(Todos::subscription)
        .font(Todos::ICON_FONT)
        .window_size((500.0, 800.0))
        .run_with(Todos::new)
}
