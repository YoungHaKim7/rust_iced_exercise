use iced::widget::{button, column, container, row, text, Button, Column, Container, Row, Text};
use iced::{
    alignment, executor, Alignment, Application, Command, Element, Length, Settings, Theme,
};

// as it wasn't used and layout is handled by Iced's widgets alignment, executor,

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
    /// Whether the 'left' value currently displayed is the result of a calculation (shadowing the actual expression)
    shadow: bool,
    error: Option<String>,
}

impl Calculator {
    // Helper to reset the calculator state
    fn reset(&mut self, initial_left: char) {
        self.left = initial_left.into();
        self.sign.clear();
        self.right.clear();
        self.shadow = false;
        self.error = None;
    }

    // Calculation logic
    fn calculate(&mut self) -> Result<(), String> {
        // Clear any previous error
        self.error = None;

        // Use parse().map_err() for better error handling instead of unwrap()
        let left_val = self
            .left
            .parse::<f64>()
            .map_err(|e| format!("Invalid left operand: {}", e))?;
        let right_val = self
            .right
            .parse::<f64>()
            .map_err(|e| format!("Invalid right operand: {}", e))?;

        let result = match self.sign.as_str() {
            "+" => Ok(left_val + right_val),
            "-" => Ok(left_val - right_val),
            "×" => Ok(left_val * right_val),
            "÷" => {
                if right_val.abs() <= std::f64::EPSILON {
                    Err("Cannot divide by zero".to_string())
                } else {
                    Ok(left_val / right_val)
                }
            }
            _ => Err(format!("Unknown operator: {}", self.sign)), // Should be unreachable with current buttons
        };

        match result {
            Ok(value) => {
                // Format to reasonably handle precision and trailing zeros
                let mut formatted = format!("{:.10}", value);
                // Remove trailing zeros after decimal point
                if formatted.contains('.') {
                    formatted = formatted.trim_end_matches('0').to_string();
                    // Remove trailing decimal point if it's the last char
                    if formatted.ends_with('.') {
                        formatted.pop();
                    }
                }
                // Handle potential "-0" case
                if formatted == "-0" {
                    formatted = "0".to_string();
                }

                self.left = formatted;
                self.right.clear(); // Prepare for next operation or number input
                self.sign.clear(); // Clear sign after calculation
                self.shadow = true; // Indicate the result is displayed
                Ok(())
            }
            Err(e) => {
                self.error = Some(e.clone()); // Store the error message
                self.shadow = false; // Show the error state, not a shadowed result
                Err(e)
            }
        }
    }
}

// Implement the Application trait for Iced
impl Application for Calculator {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = (); // No initialization flags needed

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
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

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        // If there's an error, any action except Clear should reset first
        if self.error.is_some() && message != Message::Clear {
            if let Message::Num(n) = message {
                self.reset(n); // Start new input directly if it's a number
            } else {
                self.reset('0'); // Otherwise reset to 0
                                 // We might still process the message *after* resetting if needed
                                 // For now, just reset and ignore the original message to avoid complex state
                return Command::none();
            }
        } else iome() && message == Message::Clear {
            self.reset('0'); // Clear just resets
            return Command::none();
        }

        match message {
            Message::Num(n) => {
                if !self.sign.is_empty() {
                    // Entering right operand
                    if self.shadow {
                        // Started typing a number after calculation result was shown
                        self.reset(n);
                    } else {
                        if self.right == "0" {
                            self.right = n.into();
                        } else {
                            self.right.push(n);
                        }
                    }
                } else {
                    // Entering left operand
                    if self.shadow {
                        // Started typing a number after calculation result was shown
                        self.reset(n);
                    } else if self.left == "0" {
                        self.left = n.into();
                    } else {
                        // Prevent excessively long numbers (optional)
                        // if self.left.len() < 20 {
                        self.left.push(n);
                        // }
                    }
                }
            }
            Message::Sign(s) => {
                if !self.left.is_empty() {
                    // If right operand exists, calculate the intermediate result first
                    if !self.sign.is_empty() && !self.right.is_empty() && !self.shadow {
                        if self.calculate().is_ok() {
                            self.sign = s.into(); // Set the new operator for chaining
                            self.shadow = false; // Ready for the next right operand
                        } else {
                            // Error occurred and is stored in self.error, handled above or in view
                        }
                    } else {
                        // Set the operator, clear shadow if needed
                        self.sign = s.into();
                        self.shadow = false;
                        // Ensure right side is clear if we just set the operator
                        self.right.clear();
                    }
                }
            }
            Message::Ans => {
                if !self.sign.is_empty() && !self.left.is_empty() && !self.right.is_empty() {
                    let _ = self.calculate(); // Ignore result here, state is updated internally
                                              // Error state is handled by calculate() and the check at the start of update()
                }
            }
> {
                if !self.sign.is_empty() {
                    // Dot for the right operand
                    if self.shadow {
                        // Start new number if after calculation
                        self.reset('0');
                        self.right = "0.".into(); // Start directly with "0."
                    } else if !self.right.contains('.') {
                        if self.right.is_empty() {
                            self.right.push('0');
                        }
                        self.right.push('.');
                    }
                } else {
                    // Dot for the left operand
                    if self.shadow {
                        // Start new number if after calculation
                        self.reset('0');
                        self.left = "0.".into(); // Start directly with "0."
                    } else if !self.left.contains('.') {
                        self.left.push('.');
                    }
                }
            }
            Message::Neg => {
                // Determine which operand to negate
                let target = if !self.sign.is_empty() && !self.shadow && !self.right.is_empty() {
                    &mut self.right // Negate right if sign is set, not shadow, and right exists
                } else {
                    &mut self.left // Otherwise negate left (or the shadowed result)
                };

                if target != "0" && !target.is_empty() && target != "0." {
                    if target.starts_with('-') {
                        *target = target.strip_prefix('-').unwrap().to_string();
                    } else {
                        target.insert(0, '-');
                    }
                    // If negating the shadowed result, clear the shadow flag as it's now a modified value
                    if self.shadow && std::ptr::eq(target, &self.left) {
                        self.shadow = false;
                        // Clear sign/right if user negates result? Or allow chaining?
                        // Let's allow chaining for now.
                        // self.sign.clear();
                        // self.right.clear();
                    }
                }
            }
            Message::Clear => {
                self.reset('0');
            }
            Message::ClearEnd => {
                // If there's an error, CE acts like C
                if self.error.is_some() {
                    self.reset('0');
                } else if !self.right.is_empty() {
                    self.right = "0".into(); // Clear entry to 0 instead of empty
                } else if !self.sign.is_empty() {
                    self.sign.clear(); // Clear operator if right side was already empty/0
                } else {
                    self.left = "0".into(); // Clear left entry if sign/right were already clear
                    self.shadow = false; // Ensure shadow is off
                }
            }
            Message::Backspace => {
                if self.error.is_some() {
                    self.reset('0'); // Backspace clears error state
                    return Command::none();
                }

                if self.shadow {
                    // Backspace on a result does nothing or clears? Let's make it do nothing.
                    return Command::none();
                }

                if !self.right.is_empty() {
                    self.right.pop();
                    if self.right.is_empty() || self.right == "-" {
                        self.right = "0".into(); // Reset to 0 if backspace clears it
                    }
                } else if !self.sign.is_empty() {
                    // Maybe backspace should remove the operator?
                    self.sign.clear();
                } else if !self.left.is_empty() && self.left != "0" {
                    self.left.pop();
                    if self.left.is_empty() || self.left == "-" {
                        self.left = "0".into(); // Reset to 0
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // Display Logic
        let display_text = if let Some(err) = &self.error {
            err.clone() // Show error message
        } else if self.shadow {
            self.left.to_string() // Show result
        } else {
            // Show current input expression
            let right_display = if self.right.is_empty() && !self.sign.is_empty() {
                // Show only "left sign" if right side hasn't been entered yet
                ""
            } else {
                &self.right
            };
            format!("{} {} {}", self.left, self.sign, right_display)
                .trim()
                .to_string()
        };

        let display = text(display_text)
            .size(40)
            .horizontal_alignment(alignment::Horizontal::Right)
            .width(Length::Fill); // Make display take full width

        // Helper for creating buttons
        fn calc_button(label: &str, msg: Message) -> Button<'_, Message> {
            button(
                text(label)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .width(Length::Fill), // Make text fill button
            )
            .padding(15)
            .width(Length::Fill) // Make button fill available space in row
            .on_press(msg)
        }

        // Button Layout
        let buttons = column![
            row![
                calc_button("C", Message::Clear), //.style(theme::Button::Destructive) // Optional styling
                calc_button("CE", Message::ClearEnd),
                calc_button("<-", Message::Backspace), //.style(theme::Button::Secondary)
                calc_button("÷", Message::Sign('÷')),  //.style(theme::Button::Primary)
            ]
            .spacing(10)
            .align_items(Alignment::Center),
            row![
                calc_button("7", Message::Num('7')),
                calc_button("8", Message::Num('8')),
                calc_button("9", Message::Num('9')),
                calc_button("×", Message::Sign('×')), //.style(theme::Button::Primary)
            ]
            .spacing(10)
            .align_items(Alignment::Center),
            row![
                calc_button("4", Message::Num('4')),
                calc_button("5", Message::Num('5')),
                calc_button("6", Message::Num('6')),
                calc_button("-", Message::Sign('-')), //.style(theme::Button::Primary)
            ]
            .spacing(10)
            .align_items(Alignment::Center),
            row![
                calc_button("1", Message::Num('1')),
                calc_button("2", Message::Num('2')),
                calc_button("3", Message::Num('3')),
                calc_button("+", Message::Sign('+')), //.style(theme::Button::Primary)
            ]
            .spacing(10)
            .align_items(Alignment::Center),
            row![
                calc_button("+/-", Message::Neg),
                calc_button("0", Message::Num('0')),
                calc_button(".", Message::Dot),
                calc_button("=", Message::Ans), //.style(theme::Button::Success)
            ]
            .spacing(10)
            .align_items(Alignment::Center),
        ]
        .spacing(10); // Vertical spacing between rows

        // Main content layout
        let content: Element<Message> = column![display, buttons]
            .padding(20)
            .spacing(20) // Space between display and button grid
            .align_items(Alignment::End) // Align display text to the right effectively
            .into();

        // Container to center the content (optional) and provide background
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark // Or Theme::Light, or custom theme
    }
}

// Main function using the Application trait runner
fn main() -> iced::Result {
    Calculator::run(Settings {
        window: iced::window::Settings {
            size: (350, 500), // Set a reasonable initial window size
            resizable: true,  // Allow resizing
            ..Default::default()
        },
        ..Settings::default()
    })
}
