use iced::Result;

use crate::app::todos_states::Todos;

mod app;
mod error;

const SCREEN_W: f32 = 500.0;
const SCREEN_H: f32 = 800.0;

fn main() -> Result {
    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();
    iced::application(Todos::title, Todos::update, Todos::view)
        .subscription(Todos::subscription)
        .font(Todos::ICON_FONT)
        .window_size((SCREEN_W, SCREEN_H))
        .run_with(Todos::new)
}
