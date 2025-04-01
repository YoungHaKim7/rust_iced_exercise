use iced::Center;
use iced::widget::{Column, button, column, text};

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::Zero => self.value = 0,
            Message::One => self.value = 1,
            Message::Two => self.value = 2,
            Message::Three => self.value = 3,
            Message::Four => self.value = 4,
            Message::Five => self.value = 5,
            Message::Six => self.value = 6,
            Message::Seven => self.value = 7,
            Message::Eight => self.value = 8,
            Message::Nine => self.value = 9,
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            text(self.value).size(50),
            button("Increment").on_press(Message::Increment),
            button("Decrement").on_press(Message::Decrement),
            button("9").on_press(Message::Nine),
            button("8").on_press(Message::Eight),
            button("7").on_press(Message::Seven),
            button("6").on_press(Message::Six),
            button("5").on_press(Message::Five),
            button("4").on_press(Message::Four),
            button("3").on_press(Message::Three),
            button("2").on_press(Message::Two),
            button("1").on_press(Message::One),
        ]
        .padding(20)
        .align_x(Center)
    }
}

pub fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_test::selector::text;
    use iced_test::{Error, simulator};

    #[test]
    fn it_counts() -> Result<(), Error> {
        let mut counter = Counter { value: 0 };
        let mut ui = simulator(counter.view());

        let _ = ui.click(text("Increment"))?;
        let _ = ui.click(text("Increment"))?;
        let _ = ui.click(text("Decrement"))?;

        for message in ui.into_messages() {
            counter.update(message);
        }

        assert_eq!(counter.value, 1);

        let mut ui = simulator(counter.view());
        assert!(ui.find(text("1")).is_ok(), "Counter should display 1!");

        Ok(())
    }
}
