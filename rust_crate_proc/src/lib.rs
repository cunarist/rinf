use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

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
pub fn derive_dart_signal(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl #name {
            pub fn get_dart_signal_receiver() {
                // TODO: Fill in
            }
        }
    };

    TokenStream::from(expanded)
}

/// Marks the struct as a signal endpoint
/// that contains a message from Rust to Dart.
/// This can be marked on any type that implements `Serialize`.
#[proc_macro_derive(RustSignal)]
pub fn derive_rust_signal(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl #name {
            pub fn send_signal_to_dart(self) {
                // TODO: Fill in
            }
        }
    };

    TokenStream::from(expanded)
}

// TODO: Enforce trait-based type safety
