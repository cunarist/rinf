use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
  Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Field, Fields,
  Ident, Index, Result, Variant, parse_macro_input,
};

static BANNED_LOWER_PREFIX: &str = "rinf";

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
  if name_lit.to_lowercase().starts_with(BANNED_LOWER_PREFIX) {
    return create_name_error(ast);
  }

  // Ban generic types.
  if ast.generics.params.iter().count() != 0 {
    return create_generic_error(ast);
  }

  // Check the attributes of the variants / fields.
  if let Err(error) = check_invalid_attrs(&ast.data) {
    return error.to_compile_error().into();
  }

  // Require that all included fields implement the signal trait.
  let expanded = match &ast.data {
    Data::Struct(data_struct) => get_struct_signal_impl(data_struct, name),
    Data::Enum(data_enum) => get_enum_signal_impl(data_enum, name),
    _ => return TokenStream::new(),
  };

  // Convert the generated code into token stream and return it.
  TokenStream::from(expanded)
}

/// Marks the struct as a signal endpoint
/// that contains a message from Dart to Rust.
/// This can be marked on any type that implements `Deserialize`.
#[proc_macro_derive(DartSignal)]
pub fn derive_dart_signal(input: TokenStream) -> TokenStream {
  derive_dart_signal_real(input, false)
}

/// Marks the struct as a signal endpoint
/// that contains a message and binary from Dart to Rust.
/// This can be marked on any type that implements `Deserialize`.
#[proc_macro_derive(DartSignalBinary)]
pub fn derive_dart_signal_binary(input: TokenStream) -> TokenStream {
  derive_dart_signal_real(input, true)
}

fn derive_dart_signal_real(
  input: TokenStream,
  include_binary: bool,
) -> TokenStream {
  // Collect information about the item.
  let ast = parse_macro_input!(input as DeriveInput);
  let name = &ast.ident;
  let name_lit = name.to_string();
  let snake_name = name_lit.to_snake_case();
  let upper_snake_name = name_lit.to_shouty_snake_case();

  // Check the name.
  if name_lit.to_lowercase().starts_with(BANNED_LOWER_PREFIX) {
    return create_name_error(ast);
  }

  // Ban generic types.
  if ast.generics.params.iter().count() != 0 {
    return create_generic_error(ast);
  }

  // Check the attributes of the variants / fields.
  if let Err(error) = check_invalid_attrs(&ast.data) {
    return error.to_compile_error().into();
  }

  // Require that all included fields implement the signal trait.
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
  let signal_trait = if include_binary {
    quote! { rinf::DartSignalBinary }
  } else {
    quote! { rinf::DartSignal }
  };
  let expanded = quote! {
    impl #signal_trait for #name #where_clause {
      fn get_dart_signal_receiver(
      ) -> rinf::SignalReceiver<rinf::DartSignalPack<Self>> {
        #channel_const_ident.1.clone()
      }
    }

    impl #name #where_clause {
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
  if name_lit.to_lowercase().starts_with(BANNED_LOWER_PREFIX) {
    return create_name_error(ast);
  }

  // Ban generic types.
  if ast.generics.params.iter().count() != 0 {
    return create_generic_error(ast);
  }

  // Check the attributes of the variants / fields.
  if let Err(error) = check_invalid_attrs(&ast.data) {
    return error.to_compile_error().into();
  }

  // Require that all included fields implement the signal trait.
  let where_clause = match &ast.data {
    Data::Struct(data_struct) => get_struct_where_clause(data_struct),
    Data::Enum(data_enum) => get_enum_where_clause(data_enum),
    _ => return TokenStream::new(),
  };

  // Implement methods and extern functions.
  let expanded = if include_binary {
    quote! {
      impl rinf::RustSignalBinary for #name #where_clause {
        fn send_signal_to_dart(&self, binary: Vec<u8>) {
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
      impl rinf::RustSignal for #name #where_clause {
        fn send_signal_to_dart(&self) {
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

/// Checks if the attributes of the fields are valid for Rinf signals.
fn check_fields(fields: &Fields) -> Result<()> {
  match fields {
    Fields::Named(fields) => check_attrs(&fields.named),
    Fields::Unnamed(fields) => check_attrs(&fields.unnamed),
    Fields::Unit => Ok(()),
  }
}

static BANNED_SERDE_ATTRS: [&str; 3] = [
  "skip_serializing",
  "skip_serializing_if",
  "skip_deserializing",
];

/// Checks if the attributes of a field are valid for Rinf signals.
fn check_attrs<T: GetAttrs>(items: &Punctuated<T, Comma>) -> Result<()> {
  items.iter().try_for_each(|item| {
    item.get_attrs().iter().try_for_each(|attr| {
      if !attr.path().is_ident("serde") {
        return Ok(());
      }
      attr.parse_nested_meta(|meta| match meta.path.get_ident() {
        Some(ident) => {
          // Check if the attribute is one of the banned serde attributes
          if BANNED_SERDE_ATTRS.contains(&ident.to_string().as_str()) {
            Err(meta.error(format!(
              "`{ident}` cannot be used on a field of Rinf signal"
            )))
          } else {
            Ok(())
          }
        }
        None => Ok(()),
      })
    })
  })
}

/// Checks if the attributes of the variants or fields are unsupported.
fn check_invalid_attrs(data: &Data) -> Result<()> {
  match data {
    Data::Struct(data) => check_fields(&data.fields),
    Data::Enum(data) => {
      check_attrs(&data.variants)?;
      data
        .variants
        .iter()
        .try_for_each(|variant| check_fields(&variant.fields))
    }
    Data::Union(_) => Ok(()), // Serde does not support derive for unions
  }
}

/// Require that all included fields of a struct implement the [`SignalPiece`] trait.
/// This assists with type-safe development.
fn get_struct_where_clause(
  data_struct: &DataStruct,
) -> proc_macro2::TokenStream {
  let field_types: Vec<_> = match &data_struct.fields {
    // For named structs (struct-like), extract the field types.
    Fields::Named(all) => {
      all.named.iter().filter(is_exposed).map(|f| &f.ty).collect()
    }
    // For unnamed structs (tuple-like), extract the field types.
    Fields::Unnamed(all) => all
      .unnamed
      .iter()
      .filter(is_exposed)
      .map(|f| &f.ty)
      .collect(),
    // For unit-like structs (without any inner data), do nothing.
    Fields::Unit => Vec::new(),
  };
  quote! {
    where #(#field_types: rinf::SignalPiece),*
  }
}

/// Require that all included variants of an enum implement the [`SignalPiece`] trait.
/// This assists with type-safe development.
fn get_enum_where_clause(data_enum: &DataEnum) -> proc_macro2::TokenStream {
  let variant_types: Vec<_> = data_enum
    .variants
    .iter()
    .filter(is_exposed)
    .flat_map(|variant| {
      match &variant.fields {
        // For named variants (struct-like), extract the field types.
        Fields::Named(all) => {
          all.named.iter().filter(is_exposed).map(|f| &f.ty).collect()
        }
        // For unnamed variants (tuple-like), extract the field types.
        Fields::Unnamed(all) => all
          .unnamed
          .iter()
          .filter(is_exposed)
          .map(|f| &f.ty)
          .collect(),
        // For unit-like variants (without any inner data), do nothing.
        Fields::Unit => Vec::new(),
      }
    })
    .collect();

  quote! {
    where #(#variant_types: rinf::SignalPiece),*
  }
}

fn get_struct_signal_impl(
  data_struct: &DataStruct,
  name: &Ident,
) -> proc_macro2::TokenStream {
  match &data_struct.fields {
    Fields::Named(named_fields) => {
      let fields = named_fields
        .named
        .iter()
        .filter(is_exposed)
        .filter_map(|field| field.ident.clone());
      quote! {
        impl rinf::SignalPiece for #name {
          fn be_signal_piece(&self) {
            use rinf::SignalPiece;
            #(SignalPiece::be_signal_piece(&self.#fields);)*
          }
        }
      }
    }
    Fields::Unnamed(unnamed_fields) => {
      let field_indices: Vec<Index> = unnamed_fields
        .unnamed
        .iter()
        .enumerate()
        .filter(is_exposed)
        .map(|(index, _)| Index::from(index))
        .collect();
      quote! {
        impl rinf::SignalPiece for #name {
          fn be_signal_piece(&self) {
            use rinf::SignalPiece;
            #(SignalPiece::be_signal_piece(&self.#field_indices);)*
          }
        }
      }
    }
    Fields::Unit => {
      quote! {
        impl rinf::SignalPiece for #name {
          fn be_signal_piece(&self) {
            // Unit struct has no fields to check
          }
        }
      }
    }
  }
}

fn get_enum_signal_impl(
  data_enum: &DataEnum,
  name: &Ident,
) -> proc_macro2::TokenStream {
  let variants = data_enum.variants.iter().filter(is_exposed).map(|variant| {
    let variant_ident = &variant.ident;
    match &variant.fields {
      Fields::Named(named_fields) => {
        let fields: Vec<Ident> = named_fields
          .named
          .iter()
          .filter(is_exposed)
          .filter_map(|field| field.ident.clone())
          .collect();
        quote! {
          Self::#variant_ident { #(#fields, )* .. } => {
            use rinf::SignalPiece;
            #(SignalPiece::be_signal_piece(#fields);)*
          }
        }
      }
      Fields::Unnamed(unnamed_fields) => {
        let field_vars: Vec<Ident> = unnamed_fields
          .unnamed
          .iter()
          .enumerate()
          .map(|(index, field)| match is_exposed(field) {
            true => Ident::new(&format!("field_{index}"), variant_ident.span()),
            false => Ident::new("_", variant_ident.span()),
          })
          .collect();
        let field_vars_filtered: Vec<Ident> = field_vars
          .iter()
          .filter(|&ident| ident != "_")
          .cloned()
          .collect();
        quote! {
          Self::#variant_ident(#(#field_vars),*) => {
            use rinf::SignalPiece;
            #(SignalPiece::be_signal_piece(#field_vars_filtered);)*
          }
        }
      }
      Fields::Unit => {
        quote! {
          Self::#variant_ident => {}
        }
      }
    }
  });
  quote! {
    impl rinf::SignalPiece for #name {
      fn be_signal_piece(&self) {
        match self {
          #( #variants )*
          _ => {}
        }
      }
    }
  }
}

/// Returns `false` if Serde skips the field item during serialization.
fn is_exposed<T: GetAttrs>(item: &T) -> bool {
  !item.get_attrs().iter().any(|attr| {
    if !attr.path().is_ident("serde") {
      return false;
    }
    let mut skip = false;
    let _ = attr.parse_nested_meta(|meta| {
      if meta.path.is_ident("skip") {
        skip = true;
      }
      Ok(())
    });
    skip
  })
}

/// Helper trait required for [`check_invalid_attrs`] and [`is_exposed`].
trait GetAttrs {
  fn get_attrs(&self) -> &Vec<Attribute>;
}

impl GetAttrs for Field {
  fn get_attrs(&self) -> &Vec<Attribute> {
    &self.attrs
  }
}

impl GetAttrs for &Field {
  fn get_attrs(&self) -> &Vec<Attribute> {
    &self.attrs
  }
}

impl GetAttrs for Variant {
  fn get_attrs(&self) -> &Vec<Attribute> {
    &self.attrs
  }
}

impl GetAttrs for &Variant {
  fn get_attrs(&self) -> &Vec<Attribute> {
    &self.attrs
  }
}

impl GetAttrs for (usize, &Field) {
  fn get_attrs(&self) -> &Vec<Attribute> {
    &self.1.attrs
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
      BANNED_LOWER_PREFIX
    ),
  )
  .to_compile_error()
  .into()
}
