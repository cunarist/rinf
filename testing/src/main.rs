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

fn main() {
    unsafe {
        expose_hidden();
    }
}
