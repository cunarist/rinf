use flutter_rust_bridge::SyncReturn;

pub fn request_task(order: String, json: String) -> SyncReturn<()> {
    // Dart's front-end main thread

    hub::send_task(order, json);

    SyncReturn(())
}

pub fn start_main() {
    // Rust's back-end sub thread

    hub::main();
}
