# rust_iced_exercise

https://docs.rs/iced/latest/iced/trait.Application.html

<br>

# rust iced Github

https://github.com/iced-rs/iced

<br>

# iced.rs

- https://iced.rs/
- eBook
  - https://book.iced.rs/index.html

<hr />

# iced runtime이해
- https://docs.rs/iced_runtime/latest/iced_runtime/

<br>



# rust_iced_exercise

- Rust Iced crate.io * Hello World & 한글러스트Rust강의\_045⭐️Rust*시계만들기GUI clock / #rustlang #helix #iced

https://youtu.be/4mcliazb6No


# column macro이해하기
- https://docs.rs/iced_widget/0.13.4/src/iced_widget/helpers.rs.html#32-65

```rs
/// Creates a [`Column`] with the given children.
///
/// Columns distribute their children vertically.
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::{button, column};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     column![
///         "I am on top!",
///         button("I am in the center!"),
///         "I am below.",
///     ].into()
/// }
/// ```
#[macro_export]
macro_rules! column {
    () => (
        $crate::Column::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::Column::with_children([$($crate::core::Element::from($x)),+])
    );
}


/// Creates a [`Row`] with the given children.
///
/// Rows distribute their children horizontally.
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::{button, row};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     row![
///         "I am to the left!",
///         button("I am in the middle!"),
///         "I am to the right!",
///     ].into()
/// }
/// ```
#[macro_export]
macro_rules! row {
    () => (
        $crate::Row::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::Row::with_children([$($crate::core::Element::from($x)),+])
    );
}


```
