use iced::Center;
use iced::widget::{Column, button, column, row, text};

#[derive(Default)]
struct Counter {
    value: i64,
    result: i64,
    oper: char,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Plus,
    Minus,
    Multi,
    Divide,
    Eqaul,
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
            Message::Multi => {}
            Message::Divide => {}
            Message::Minus => {
                self.value -= 1;
            }
            Message::Plus => {
                self.value += 1;
            }
            Message::Eqaul => {}
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
        let screen = text(self.value).size(100);
        let row_1 = row![
            button("/").on_press(Message::Divide),
            button("*").on_press(Message::Multi),
            button("+").on_press(Message::Plus),
            button("-").on_press(Message::Minus),
            button("=").on_press(Message::Eqaul),
        ];
        column![
            screen,
            row_1,
            row![
                button("7").on_press(Message::Seven),
                button("8").on_press(Message::Eight),
                button("9").on_press(Message::Nine),
            ],
            row![
                button("4").on_press(Message::Four),
                button("5").on_press(Message::Five),
                button("6").on_press(Message::Six),
            ],
            row![
                button("1").on_press(Message::One),
                button("2").on_press(Message::Two),
                button("3").on_press(Message::Three),
            ],
            row![button("0").on_press(Message::Zero),],
        ]
        .padding(20)
        .spacing(1)
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
