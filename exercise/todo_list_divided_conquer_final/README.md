# Result


- nightly backtrace debugging

```bash
$ RUSTFLAGS="-Zmacro-backtrace" cargo r
```

- https://github.com/rustwasm/wasm-pack/issues/666
- https://github.com/rust-lang/cargo/issues/7233


- Describe the problem you are trying to solve
Currently `cargo test --target wasm32-unkown-unkown` will try to compile all examples. This is a problem if there is code that uses dev-dependencies that are not available on this target, like networking crates.

- Describe the solution you'd like
This already works for files in the tests directory, but not for examples:
I would like to see an example ignored when it has: `#![ cfg(not( target_arch = "wasm32" )) ]` at the top of the file.

- Currently this throws an error like:

- I made some progress without modifying cargo:

- I tried `package.autoexamples = false + [[target."cfg(not( target_arch = \"wasm32\" ))".example]]` but that failed. It seems that the target selection is specific to dependencies?
 - I managed a workaround with `package.autoexamples = false`, a notwasm feature and using required-features = notwasm in the example section. The downside of this is that users can only run examples with: `cargo run --example basic --features notwasm`, which is unfortunate and also one needs to list all examples in `Cargo.toml`. I suppose this technique could also work with a nightly feature to allow tests to run in stable

# Hey Rustaceans! Got a question? Ask here (16/2023)! 
- https://www.reddit.com/r/rust/comments/12p5xh4/hey_rustaceans_got_a_question_ask_here_162023/?rdt=45360
