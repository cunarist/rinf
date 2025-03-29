use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
  Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Ident,
  parse_macro_input,
};

static BANNED_PREFIX: &str = "Rinf";

/// Marks the struct as a signal
/// that can be nested within other signals.
/// A `SignalPiece` cannot be sent independently
/// and is only a partial component of `DartSignal` or `RustSignal`.
#[proc_macro_derive(SignalPiece)]
pub fn derive_signal_piece(input: TokenStream) -> TokenStream {
  // Collect information about the item.
  let ast = parse_macro_input!(input as DeriveInput);
  let name = &ast.ident;
  let name_lit = name.to_string();

  // Check the name.
  if name_lit.starts_with(BANNED_PREFIX) {
    return create_name_error(ast);
  }

  // Ban generic types.
  if ast.generics.params.iter().count() != 0 {
    return create_generic_error(ast);
  }

  // Enforce all fields to implement the foreign signal trait.
  let where_clause = match &ast.data {
    Data::Struct(data_struct) => get_struct_where_clause(data_struct),
    Data::Enum(data_enum) => get_enum_where_clause(data_enum),
    _ => return TokenStream::new(),
  };

  // Automatically implement the signal trait for the struct.
  let expanded = quote! {
    impl rinf::ForeignSignal for #name #where_clause {}
  };

  // Convert the generated code into token stream and return it.
  TokenStream::from(expanded)
}

/// Marks the struct as a signal endpoint
/// that contains a message from Dart to Rust.
/// This can be marked on any type that implements `Deserialize`.
#[proc_macro_derive(DartSignal)]
pub fn derive_dart_signal(input: TokenStream) -> TokenStream {
  derive_dart_signal_real(input)
}

/// Marks the struct as a signal endpoint
/// that contains a message and binary from Dart to Rust.
/// This can be marked on any type that implements `Deserialize`.
#[proc_macro_derive(DartSignalBinary)]
pub fn derive_dart_signal_binary(input: TokenStream) -> TokenStream {
  derive_dart_signal_real(input)
}

fn derive_dart_signal_real(input: TokenStream) -> TokenStream {
  // Collect information about the item.
  let ast = parse_macro_input!(input as DeriveInput);
  let name = &ast.ident;
  let name_lit = name.to_string();
  let snake_name = name_lit.to_case(Case::Snake);
  let upper_snake_name = name_lit.to_case(Case::UpperSnake);

  // Check the name.
  if name_lit.starts_with(BANNED_PREFIX) {
    return create_name_error(ast);
  }

  // Ban generic types.
  if ast.generics.params.iter().count() != 0 {
    return create_generic_error(ast);
  }

  // Enforce all fields to implement the foreign signal trait.
  let where_clause = match &ast.data {
    Data::Struct(data_struct) => get_struct_where_clause(data_struct),
    Data::Enum(data_enum) => get_enum_where_clause(data_enum),
    _ => return TokenStream::new(),
  };

  // Collect identifiers and names.
  let channel_type_ident = Ident::new(&format!("{}Channel", name), name.span());
  let channel_const_ident =
    Ident::new(&format!("{}_CHANNEL", upper_snake_name), name.span());
  let extern_fn_name = &format!("rinf_send_dart_signal_{}", snake_name);
  let extern_fn_ident = Ident::new(extern_fn_name, name.span());

  // Implement methods and extern functions.
  let expanded = quote! {
    impl #name #where_clause {
      /// Gets the receiver that listens for signals from Dart.
      /// If this function is called multiple times,
      /// only the last receiver remains alive,
      /// and all previous ones become inactive after receiving `None`.
      pub fn get_dart_signal_receiver(
      ) -> rinf::SignalReceiver<rinf::DartSignalPack<Self>> {
        #channel_const_ident.1.clone()
      }

      fn send_dart_signal(message_bytes: &[u8], binary: &[u8]) {
        use rinf::{AppError, DartSignalPack, debug_print, deserialize};
        let message_result: Result<#name, AppError> =
          deserialize(message_bytes)
          .map_err(|_| AppError::CannotDecodeMessage);
        let message = match message_result {
          Ok(inner) => inner,
          Err(err) => {
            let type_name = #name_lit;
            debug_print!("{}: \n{}", type_name, err);
            return;
          }
        };
        let dart_signal = DartSignalPack {
          message,
          binary: binary.to_vec(),
        };
        #channel_const_ident.0.send(dart_signal);
      }
    }

    type #channel_type_ident = std::sync::LazyLock<(
      rinf::SignalSender<rinf::DartSignalPack<#name>>,
      rinf::SignalReceiver<rinf::DartSignalPack<#name>>,
    )>;

    static #channel_const_ident: #channel_type_ident =
      std::sync::LazyLock::new(rinf::signal_channel);

    #[cfg(not(target_family = "wasm"))]
    #[unsafe(no_mangle)]
    unsafe extern "C" fn #extern_fn_ident(
      message_pointer: *const u8,
      message_size: usize,
      binary_pointer: *const u8,
      binary_size: usize,
    ) {
      use std::slice::from_raw_parts;
      let message_bytes = from_raw_parts(message_pointer, message_size);
      let binary = from_raw_parts(binary_pointer, binary_size);
      #name::send_dart_signal(message_bytes, binary);
    }

    #[cfg(target_family = "wasm")]
    #[wasm_bindgen::prelude::wasm_bindgen]
    pub fn #extern_fn_ident(message_bytes: &[u8], binary: &[u8]) {
      #name::send_dart_signal(message_bytes, binary);
    }
  };

  // Convert the generated code into token stream and return it.
  TokenStream::from(expanded)
}

/// Marks the struct as a signal endpoint
/// that contains a message from Rust to Dart.
/// This can be marked on any type that implements `Serialize`.
#[proc_macro_derive(RustSignal)]
pub fn derive_rust_signal(input: TokenStream) -> TokenStream {
  derive_rust_signal_real(input, false)
}

/// Marks the struct as a signal endpoint
/// that contains a message and binary from Rust to Dart.
/// This can be marked on any type that implements `Serialize`.
#[proc_macro_derive(RustSignalBinary)]
pub fn derive_rust_signal_binary(input: TokenStream) -> TokenStream {
  derive_rust_signal_real(input, true)
}

fn derive_rust_signal_real(
  input: TokenStream,
  include_binary: bool,
) -> TokenStream {
  // Collect information about the item.
  let ast = parse_macro_input!(input as DeriveInput);
  let name = &ast.ident;
  let name_lit = name.to_string();

  // Check the name.
  if name_lit.starts_with(BANNED_PREFIX) {
    return create_name_error(ast);
  }

  // Ban generic types.
  if ast.generics.params.iter().count() != 0 {
    return create_generic_error(ast);
  }

  // Enforce all fields to implement the foreign signal trait.
  let where_clause = match &ast.data {
    Data::Struct(data_struct) => get_struct_where_clause(data_struct),
    Data::Enum(data_enum) => get_enum_where_clause(data_enum),
    _ => return TokenStream::new(),
  };

  // Implement methods and extern functions.
  let expanded = if include_binary {
    quote! {
      impl #name #where_clause {
        /// Sends the signal to Dart with separate binary data.
        /// Passing data from Dart to Rust is a zero-copy operation.
        pub fn send_signal_to_dart(self, binary: Vec<u8>) {
          use rinf::{AppError, debug_print, send_rust_signal, serialize};
          let type_name = #name_lit;
          let message_result: Result<Vec<u8>, AppError> =
            serialize(&self)
            .map_err(|_| AppError::CannotEncodeMessage);
          let message_bytes = match message_result {
            Ok(inner) => inner,
            Err(err) => {
              debug_print!("{}: \n{}", type_name, err);
              return;
            }
          };
          let result = send_rust_signal(type_name, message_bytes, binary);
          if let Err(err) = result {
            debug_print!("{}: \n{}", type_name, err);
          }
        }
      }
    }
  } else {
    quote! {
      impl #name #where_clause {
        /// Sends the signal to Dart.
        /// Passing data from Dart to Rust is a zero-copy operation.
        pub fn send_signal_to_dart(self) {
          use rinf::{AppError, debug_print, send_rust_signal, serialize};
          let type_name = #name_lit;
          let message_result: Result<Vec<u8>, AppError> =
            serialize(&self)
            .map_err(|_| AppError::CannotEncodeMessage);
          let message_bytes = match message_result {
            Ok(inner) => inner,
            Err(err) => {
              debug_print!("{}: \n{}", type_name, err);
              return;
            }
          };
          let result = send_rust_signal(type_name, message_bytes, Vec::new());
          if let Err(err) = result {
            debug_print!("{}: \n{}", type_name, err);
          }
        }
      }
    }
  };

  // Convert the generated code into token stream and return it.
  TokenStream::from(expanded)
}

/// Enforces all fields of a struct to have the foreign signal trait.
/// This assists with type-safe development.
fn get_struct_where_clause(
  data_struct: &DataStruct,
) -> proc_macro2::TokenStream {
  let field_types: Vec<_> = match &data_struct.fields {
    // For named structs (struct-like), extract the field types.
    Fields::Named(all) => all.named.iter().map(|f| &f.ty).collect(),
    // For unnamed structs (tuple-like), extract the field types.
    Fields::Unnamed(all) => all.unnamed.iter().map(|f| &f.ty).collect(),
    // For unit-like structs (without any inner data), do nothing.
    Fields::Unit => Vec::new(),
  };
  quote! {
    where #(#field_types: rinf::ForeignSignal),*
  }
}

/// Enforces all fields of an enum variant to have the foreign signal trait.
/// This assists with type-safe development.
fn get_enum_where_clause(data_enum: &DataEnum) -> proc_macro2::TokenStream {
  let variant_types: Vec<_> = data_enum
    .variants
    .iter()
    .flat_map(|variant| {
      match &variant.fields {
        // For named variants (struct-like), extract the field types.
        Fields::Named(all) => all.named.iter().map(|f| &f.ty).collect(),
        // For unnamed variants (tuple-like), extract the field types.
        Fields::Unnamed(all) => all.unnamed.iter().map(|f| &f.ty).collect(),
        // For unit-like variants (without any inner data), do nothing.
        Fields::Unit => Vec::new(),
      }
    })
    .collect();

  quote! {
    where #(#variant_types: rinf::ForeignSignal),*
  }
}

fn create_generic_error(ast: DeriveInput) -> TokenStream {
  Error::new_spanned(ast.generics, "A foreign signal type cannot be generic")
    .to_compile_error()
    .into()
}

fn create_name_error(ast: DeriveInput) -> TokenStream {
  Error::new_spanned(
    ast.ident,
    format!(
      "The name of a foreign signal cannot start with `{}`",
      BANNED_PREFIX
    ),
  )
  .to_compile_error()
  .into()
}
