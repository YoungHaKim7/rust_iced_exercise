use iced::Result;

use crate::app::todos_states::Todos;

mod app;
mod error;

fn main() -> Result {
    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();
    iced::application(Todos::title, Todos::update, Todos::view)
        .subscription(Todos::subscription)
        .font(Todos::ICON_FONT)
        .window_size((500.0, 800.0))
        .run_with(Todos::new)
}
