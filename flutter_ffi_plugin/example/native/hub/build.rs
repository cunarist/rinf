use std::env;

fn main() {
    // https://kazlauskas.me/entries/writing-proper-buildrs-scripts
    // https://github.com/RustAudio/rodio/issues/404#issuecomment-1288096846
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("android") => {
            println!("cargo:rustc-link-lib=dylib=stdc++");
            println!("cargo:rustc-link-lib=c++_shared");
        }
        _ => {}
    }
}
