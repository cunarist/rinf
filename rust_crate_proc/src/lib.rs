use proc_macro::TokenStream;

/// Marks the struct as a signal
/// that can be nested within other signals.
#[proc_macro_derive(Signal)]
pub fn derive_signal(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// Marks the struct as a signal endpoint
/// that contains a message from Dart to Rust.
/// This can be marked on any type that implements `Deserialize`.
#[proc_macro_derive(DartSignal)]
pub fn derive_dart_signal(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// Marks the struct as a signal endpoint
/// that contains a message from Rust to Dart.
/// This can be marked on any type that implements `Serialize`.
#[proc_macro_derive(RustSignal)]
pub fn derive_rust_signal(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

// TODO: Enforce trait-based type safety
