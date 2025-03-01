mod parse;

pub use parse::generate_dart_code;

mod outer {
    mod inner {
        #[no_mangle]
        pub extern "Rust" fn expose_hidden() {
            println!("Hello from hidden function!");
        }
    }
}

extern "Rust" {
    fn expose_hidden();
}

unsafe fn test_hidden() {
    expose_hidden();
}
