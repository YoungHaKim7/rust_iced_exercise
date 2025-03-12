// use iced::Task as Command;
// use iced::alignment::Horizontal;
// use iced::widget::{Column, Text};
// use iced::{Alignment, Element, Length};

// const LEN: u32 = 800;

// #[derive(Debug, Clone, Copy)]
// pub enum Message {
//     Num(char),
//     Sign(char),
//     Ans,
//     Dot,
//     Neg,
//     Clear,
//     ClearEnd,
//     Backspace,
// }

// #[derive(Default)]
// struct Calculator {
//     left: String,
//     right: String,
//     sign: String,
//     shadow: bool,
// }

// impl Calculator {
//     fn calculate(&mut self) -> std::result::Result<(), &'static str> {
//         self.shadow = true;
//         self.left = match self.sign.as_str() {
//             "+" => format!(
//                 "{:.10}",
//                 self.left.parse::<f64>().unwrap() + self.right.parse::<f64>().unwrap()
//             ),
//             "-" => format!(
//                 "{:.10}",
//                 self.left.parse::<f64>().unwrap() - self.right.parse::<f64>().unwrap()
//             ),
//             "×" => format!(
//                 "{:.10}",
//                 self.left.parse::<f64>().unwrap() * self.right.parse::<f64>().unwrap()
//             ),
//             "÷" => {
//                 let r = self.right.parse::<f64>().unwrap();
//                 if r.abs() <= std::f64::EPSILON {
//                     return Err("Can't divide by zero");
//                 } else {
//                     format!("{:.10}", self.left.parse::<f64>().unwrap() / r)
//                 }
//             }
//             _ => unreachable!(),
//         };
//         while self.left.len() > 1 {
//             if let Some(a) = self.left.pop() {
//                 if a != '0' && a != '.' {
//                     self.left.push(a);
//                     break;
//                 }
//             }
//         }
//         Ok(())
//     }

//     #[inline]
//     fn clear(&mut self, c: char) {
//         self.left = c.into();
//         self.sign.clear();
//         self.right.clear();
//         self.shadow = false;
//     }
// }

// impl Calculator {
//     fn new(_flags: ()) -> (Self, Command<Message>) {
//         (
//             Self {
//                 left: "0".into(),
//                 ..Self::default()
//             },
//             Command::none(),
//         )
//     }

//     fn title(&self) -> String {
//         "简单计算器 ~ A Simple Calculator".into()
//     }

//     fn update(&mut self, message: Message) -> Command<Message> {
//         match message {
//             Message::Num(n) => {
//                 if self.sign.is_empty() {
//                     if &self.left == "0" {
//                         self.left = n.into();
//                     } else {
//                         self.left.push(n);
//                     }
//                 } else if !self.left.is_empty() && self.shadow {
//                     self.clear(n);
//                 } else if &self.right == "0" {
//                     self.right = n.into();
//                 } else {
//                     self.right.push(n);
//                 }
//             }
//             Message::Ans => {
//                 if !self.sign.is_empty() && !self.left.is_empty() && !self.right.is_empty() {
//                     if let Err(e) = self.calculate() {
//                         self.left = e.to_string();
//                         self.sign.clear();
//                         self.right.clear();
//                         self.shadow = false;
//                     }
//                 }
//             }
//             Message::Sign(s) => {
//                 if self.sign.is_empty() {
//                     self.sign.push(s);
//                 } else {
//                     if !self.shadow && !self.right.is_empty() {
//                         if let Err(e) = self.calculate() {
//                             self.left = e.to_string();
//                             self.sign.clear();
//                             self.right.clear();
//                             self.shadow = false;
//                             return Command::none();
//                         }
//                     }
//                     self.sign = s.into();
//                     self.right.clear();
//                     self.shadow = false;
//                 }
//             }
//             Message::ClearEnd => {
//                 if self.right.is_empty() {
//                     self.left = "0".into();
//                     self.sign.clear();
//                     self.shadow = false;
//                 } else {
//                     self.right.clear();
//                 }
//             }
//             Message::Clear => {
//                 self.clear('0');
//             }
//             Message::Backspace => {
//                 if self.sign.is_empty() {
//                     self.left.pop();
//                     if self.left.is_empty() {
//                         self.left.push('0');
//                     }
//                 } else {
//                     self.right.pop();
//                 }
//             }
//             Message::Dot => {
//                 if self.sign.is_empty() && !self.left.contains('.') {
//                     self.left.push('.');
//                 } else if !self.sign.is_empty() && !self.right.contains('.') {
//                     if self.right.is_empty() {
//                         self.right.push('0');
//                     }
//                     self.right.push('.');
//                 }
//             }
//             Message::Neg => {
//                 if self.sign.is_empty() {
//                     if &self.left != "0" && &self.left != "0." {
//                         if !self.left.starts_with('-') {
//                             self.left.insert(0, '-');
//                         } else {
//                             self.left.remove(0);
//                         }
//                     }
//                 } else if self.shadow {
//                     if &self.left != "0" {
//                         if !self.left.starts_with('-') {
//                             self.left.insert(0, '-');
//                         } else {
//                             self.left.remove(0);
//                         }
//                     }
//                 } else if &self.right != "0" && &self.right != "0." {
//                     if !self.right.starts_with('-') {
//                         self.right.insert(0, '-');
//                     } else {
//                         self.right.remove(0);
//                     }
//                 }
//             }
//         }
//         Command::none()
//     }

//     fn view(&self) -> Element<'_, Message> {
//         Column::new()
//             .max_width(LEN)
//             .width(Length::Fill)
//             .spacing(2)
//             .padding(8)
//             .align_items(Alignment::Center)
//             .push(
//                 Text::new(if self.shadow {
//                     self.left.to_string()
//                 } else {
//                     format!("{} {} {}", self.left, self.sign, self.right)
//                 })
//                 .size(50)
//                 .width(Length::Fill)
//                 .horizontal_alignment(Horizontal::Right),
//             )
//             .into()
//     }
// }

// mod style {
//     use iced::widget::button;
//     use iced::{Background, Color, Vector};

//     #[derive(Clone, Copy)]
//     pub enum ButtonStyle {
//         Ans,
//         Num,
//         Func,
//     }

//     impl ButtonStyle {
//         fn active(&self, _style: &()) -> button::Appearance {
//             button::Appearance {
//                 background: Some(Background::Color(match self {
//                     ButtonStyle::Ans => Color::from_rgb8(69, 153, 219),
//                     ButtonStyle::Num => Color::from_rgb8(242, 243, 242),
//                     ButtonStyle::Func => Color::from_rgb8(216, 218, 217),
//                 })),
//                 border_radius: 5.0,
//                 shadow_offset: Vector::new(1.0, 1.0),
//                 text_color: match self {
//                     ButtonStyle::Ans => Color::WHITE,
//                     _ => Color::BLACK,
//                 },
//                 border_width: 0.0,
//                 border_color: Color::TRANSPARENT,
//             }
//         }

//         fn hovered(&self, _style: &()) -> button::Appearance {
//             let mut appearance = self.active(_style);
//             appearance.shadow_offset = Vector::new(2.0, 2.0); // Slightly larger shadow
//             appearance
//         }

//         fn pressed(&self, _style: &()) -> button::Appearance {
//             let mut appearance = self.active(_style);
//             appearance.shadow_offset = Vector::new(0.5, 0.5); // Less shadow when pressed
//             appearance
//         }
//     }
// }

// fn main() -> iced::Result {
//     iced::application(Calculator::title, Calculator::update, Calculator::view)
//         .window_size((800.0, 300.0))
//         .run_with(Calculator::new)
// }

use iced::alignment::Horizontal;
use iced::executor;
use iced::widget::{Column, Text};
use iced::{Alignment, Length};
use iced::{Application, Command, Element, Settings, Theme};

const LEN: u32 = 800;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Num(char),
    Sign(char),
    Ans,
    Dot,
    Neg,
    Clear,
    ClearEnd,
    Backspace,
}

#[derive(Default)]
struct Calculator {
    left: String,
    right: String,
    sign: String,
    shadow: bool,
}

impl Calculator {
    fn calculate(&mut self) -> std::result::Result<(), &'static str> {
        self.shadow = true;
        self.left = match self.sign.as_str() {
            "+" => format!(
                "{:.10}",
                self.left.parse::<f64>().unwrap() + self.right.parse::<f64>().unwrap()
            ),
            "-" => format!(
                "{:.10}",
                self.left.parse::<f64>().unwrap() - self.right.parse::<f64>().unwrap()
            ),
            "×" => format!(
                "{:.10}",
                self.left.parse::<f64>().unwrap() * self.right.parse::<f64>().unwrap()
            ),
            "÷" => {
                let r = self.right.parse::<f64>().unwrap();
                if r.abs() <= std::f64::EPSILON {
                    return Err("Can't divide by zero");
                } else {
                    format!("{:.10}", self.left.parse::<f64>().unwrap() / r)
                }
            }
            _ => unreachable!(),
        };
        while self.left.len() > 1 {
            if let Some(a) = self.left.pop() {
                if a != '0' && a != '.' {
                    self.left.push(a);
                    break;
                }
            }
        }
        Ok(())
    }

    #[inline]
    fn clear(&mut self, c: char) {
        self.left = c.into();
        self.sign.clear();
        self.right.clear();
        self.shadow = false;
    }
}

impl Application for Calculator {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                left: "0".into(),
                ..Self::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "简单计算器 ~ A Simple Calculator".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Num(n) => {
                if self.sign.is_empty() {
                    if &self.left == "0" {
                        self.left = n.into();
                    } else {
                        self.left.push(n);
                    }
                } else if !self.left.is_empty() && self.shadow {
                    self.clear(n);
                } else if &self.right == "0" {
                    self.right = n.into();
                } else {
                    self.right.push(n);
                }
            }
            Message::Ans => {
                if !self.sign.is_empty() && !self.left.is_empty() && !self.right.is_empty() {
                    if let Err(e) = self.calculate() {
                        self.left = e.to_string();
                        self.sign.clear();
                        self.right.clear();
                        self.shadow = false;
                    }
                }
            }
            Message::Sign(s) => {
                if self.sign.is_empty() {
                    self.sign.push(s);
                } else {
                    if !self.shadow && !self.right.is_empty() {
                        if let Err(e) = self.calculate() {
                            self.left = e.to_string();
                            self.sign.clear();
                            self.right.clear();
                            self.shadow = false;
                            return Command::none();
                        }
                    }
                    self.sign = s.into();
                    self.right.clear();
                    self.shadow = false;
                }
            }
            Message::ClearEnd => {
                if self.right.is_empty() {
                    self.left = "0".into();
                    self.sign.clear();
                    self.shadow = false;
                } else {
                    self.right.clear();
                }
            }
            Message::Clear => {
                self.clear('0');
            }
            Message::Backspace => {
                if self.sign.is_empty() {
                    self.left.pop();
                    if self.left.is_empty() {
                        self.left.push('0');
                    }
                } else {
                    self.right.pop();
                }
            }
            Message::Dot => {
                if self.sign.is_empty() && !self.left.contains('.') {
                    self.left.push('.');
                } else if !self.sign.is_empty() && !self.right.contains('.') {
                    if self.right.is_empty() {
                        self.right.push('0');
                    }
                    self.right.push('.');
                }
            }
            Message::Neg => {
                if self.sign.is_empty() {
                    if &self.left != "0" && &self.left != "0." {
                        if !self.left.starts_with('-') {
                            self.left.insert(0, '-');
                        } else {
                            self.left.remove(0);
                        }
                    }
                } else if self.shadow {
                    if &self.left != "0" {
                        if !self.left.starts_with('-') {
                            self.left.insert(0, '-');
                        } else {
                            self.left.remove(0);
                        }
                    }
                } else if &self.right != "0" && &self.right != "0." {
                    if !self.right.starts_with('-') {
                        self.right.insert(0, '-');
                    } else {
                        self.right.remove(0);
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        Column::new()
            .max_width(LEN)
            .width(Length::Fill)
            .spacing(2)
            .padding(8)
            .align_items(Alignment::Center)
            .push(
                Text::new(if self.shadow {
                    self.left.to_string()
                } else {
                    format!("{} {} {}", self.left, self.sign, self.right)
                })
                .size(50)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Right),
            )
            .into()
    }
}

mod style {
    use iced::widget::button;
    use iced::{Background, Color, Vector};

    #[derive(Clone, Copy)]
    pub enum ButtonStyle {
        Ans,
        Num,
        Func,
    }

    impl button::StyleSheet for ButtonStyle {
        type Style = ();

        fn active(&self, _style: &()) -> button::Appearance {
            button::Appearance {
                background: Some(Background::Color(match self {
                    ButtonStyle::Ans => Color::from_rgb8(69, 153, 219),
                    ButtonStyle::Num => Color::from_rgb8(242, 243, 242),
                    ButtonStyle::Func => Color::from_rgb8(216, 218, 217),
                })),
                border_radius: 5.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: match self {
                    ButtonStyle::Ans => Color::WHITE,
                    _ => Color::BLACK,
                },
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        }
    }
}

fn main() -> iced::Result {
    Calculator::run(Settings::default())
}
