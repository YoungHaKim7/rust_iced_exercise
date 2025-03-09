# Result

- 오류 고쳐야함.  ㅠㅠ


```bash

error[E0308]: mismatched types
   --> src/main.rs:148:46
    |
148 |                 Command::batch(vec![command, save])
    |                                              ^^^^ expected `Task<Mode>`, found `Task<Message>`
    |
    = note: expected struct `iced::Task<Mode>`
               found struct `iced::Task<Message>`

For more information about this error, try `rustc --explain E0308`.
warning: `todo_list_mercury_coder_test` (bin "todo_list_mercury_coder_test") generated 1 warning
error: could not compile `todo_list_mercury_coder_test` (bin "todo_list_mercury_coder_test") due to 1 previous error; 1 warning emitted
```

