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
