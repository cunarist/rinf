# Printing for Debugging

You might be used to `println!` macro in Rust. However, using that macro isn't a very good idea in our apps made with Flutter and Rust because `stdout` stream cannot be seen on the web and mobile emulators.

When writing Rust code in the `hub` crate, you can simply print your debug message with the `debug_print!` macro provided by this framework like below. Once you use this macro, Flutter will take care of the rest.

```{code-block} rust
:caption: Rust
use rinf::debug_print;
debug_print!("My object is {:?}", my_object);
```

`debug_print!` only writes the code in debug mode, resulting in a smaller and cleaner release binary.
