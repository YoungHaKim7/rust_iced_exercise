# Result

- 오류 고쳐야함.  ㅠㅠ

```bash


error[E0433]: failed to resolve: `Center` is a variant, not a module
   --> src/main.rs:381:5
    |
381 |     Center::new(
    |     ^^^^^^ `Center` is a variant, not a module
    |
help: there is an enum variant `core::fmt::Alignment::Center` and 9 others; try using the variant's enum
    |
381 -     Center::new(
381 +     core::fmt::Alignment::new(
    |
381 -     Center::new(
381 +     crate::Alignment::new(
    |
381 -     Center::new(
381 +     crate::widget::pane_grid::Region::new(
    |
381 -     Center::new(
381 +     iced::new(
    |
      and 5 other candidates

error[E0433]: failed to resolve: `Center` is a variant, not a module
   --> src/main.rs:391:5
    |
391 |     Center::new(
    |     ^^^^^^ `Center` is a variant, not a module
    |
help: there is an enum variant `core::fmt::Alignment::Center` and 9 others; try using the variant's enum
    |
391 -     Center::new(
391 +     core::fmt::Alignment::new(
    |
391 -     Center::new(
391 +     crate::Alignment::new(
    |
391 -     Center::new(
391 +     crate::widget::pane_grid::Region::new(
    |
391 -     Center::new(
391 +     iced::new(
    |
      and 5 other candidates

error[E0433]: failed to resolve: `Center` is a variant, not a module
   --> src/main.rs:201:17
    |
201 |                 Center::new(content).into()
    |                 ^^^^^^ `Center` is a variant, not a module
    |
help: there is an enum variant `core::fmt::Alignment::Center` and 9 others; try using the variant's enum
    |
201 -                 Center::new(content).into()
201 +                 core::fmt::Alignment::new(content).into()
    |
201 -                 Center::new(content).into()
201 +                 crate::Alignment::new(content).into()
    |
201 -                 Center::new(content).into()
201 +                 crate::widget::pane_grid::Region::new(content).into()
    |
201 -                 Center::new(content).into()
201 +                 iced::new(content).into()
    |
      and 5 other candidates

error[E0308]: mismatched types
   --> src/main.rs:148:46
    |
148 |                 Command::batch(vec![command, save])
    |                                              ^^^^ expected `Task<Mode>`, found `Task<Message>`
    |
    = note: expected struct `iced::Task<Mode>`
               found struct `iced::Task<Message>`

Some errors have detailed explanations: E0308, E0433.
For more information about an error, try `rustc --explain E0308`.
warning: `todo_list_mercury_coder_test` (bin "todo_list_mercury_coder_test") generated 1 warning
error: could not compile `todo_list_mercury_coder_test` (bin "todo_list_mercury_coder_test") due to 4 previous errors; 1 warning emitted
```

