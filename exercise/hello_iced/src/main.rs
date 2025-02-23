// use iced::Element;

// #[derive(Debug, Default)]
// struct Data {
//     radius: f32,
// }

// #[derive(Debug)]
// enum Message {
//     RadiusChanged(f32),
// }

// impl Data {
//     fn new() -> Self {
//         Data { radius: 0.0 }
//     }

//     fn update(&mut self, message: Message) {
//         match message {
//             Message::RadiusChanged(radius) => {
//                 self.radius = radius;
//             }
//         }
//     }

//     fn view(&self) -> Element<Message> {
//         let content = coloum![
//             circle(self.radius),
//             text!("Radius: {:.2}", self.radius),
//             slider(1.0..=100.0, self.radius, Message::Ra)
//         ]
//     }
// }

// fn main() {
//     iced::run("Hello world - Iced", Data::update, Data::view)
// }

// use iced::subscription::run;
// use iced::widget::{Column, button, column, text};

// pub fn main() -> iced::Result {
//     // run("A counter", update, view)
//     run("A counter");
//     Ok(())
// }

// #[derive(Debug, Clone)]
// enum Message {
//     Increment,
// }

// fn update(value: &mut u64, message: Message) {
//     match message {
//         Message::Increment => *value += 1,
//     }
// }

// fn view(value: &u64) -> Column<Message> {
//     column![text(value), button("+").on_press(Message::Increment),]
// }

use iced::widget::{Column, button, column, text};
use iced::{Application, Command, Element, Settings, Subscription};

pub fn main() -> iced::Result {
    Counter::run(Settings::with_flags("A counter"))
}

struct Counter {
    value: u64,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

impl Application for Counter {
    type Message = Message;
    type Flags = &'static str;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Counter { value: 0 }, Command::none())
    }

    fn title(&self) -> String {
        String::from("A counter")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Increment => {
                self.value += 1;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            text(self.value.to_string()).size(50),
            button("+").on_press(Message::Increment),
        ]
        .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
}
