use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("wasm32") {
        println!("cargo:rustc-cfg=wasm");
    }
}
