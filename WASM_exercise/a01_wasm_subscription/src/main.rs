// use iced::{Element, Renderer, Theme, time::Duration, widget::text};

// #[derive(Debug)]
// pub enum Message {
//     Increment,
// }

// #[derive(Default)]
// pub struct App {
//     counter: usize,
// }

// impl App {
//     fn subscription(&self) -> iced::Subscription<Message> {
//         iced::time(Duration::from_millis(1000)).map(|_| Message::Increment)
//     }

//     fn update(&mut self, event: Message) {
//         match event {
//             Message::Increment => {
//                 self.counter += 1;
//             }
//         }
//     }

//     fn view(&self) -> Element<Message, Theme, Renderer> {
//         text(format!("Counter: {}", self.counter)).size(50).into()
//     }
// }

// fn main() -> iced::Result {
//     iced::application("Subscription Bug", App::update, App::view)
//         .subscription(App::subscription)
//         .run()
// }
use iced::time::{self, Duration};
use iced::widget::text;
use iced::{Application, Element, Renderer, Settings, Subscription, Theme};

use std::process::Command;

fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
}

#[derive(Default)]
pub struct App {
    counter: usize,
}

// Implement the Application trait for App
impl iced::Application for App {
    type Message = Message;
    type Flags = ();
    type Executor = iced::executor::Default;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (App::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Subscription Example")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Increment => {
                self.counter += 1;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message, Self::Theme, Renderer> {
        text(format!("Counter: {}", self.counter)).size(50).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::time::every(Duration::from_secs(1)).map(|_| Message::Increment)
    }
}
