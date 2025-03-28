use crate::tool::{RinfConfig, SetupError};
use convert_case::{Case, Casing};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde_generate::dart::{CodeGenerator, Installer};
use serde_generate::{CodeGeneratorConfig, Encoding, SourceInstaller};
use serde_reflection::{
  ContainerFormat, Format, Named, Registry, VariantFormat,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{
  create_dir_all, read_dir, read_to_string, remove_dir_all, rename, write,
};
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;
use syn::spanned::Spanned;
use syn::{
  Attribute, Expr, File, GenericArgument, Item, ItemEnum, ItemStruct, Lit,
  PathArguments, Type, TypeArray, TypePath, TypeTuple,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
enum SignalAttribute {
  SignalPiece,
  DartSignal,
  DartSignalBinary,
  RustSignal,
  RustSignalBinary,
}

fn extract_signal_attribute(
  attrs: &[Attribute],
) -> Result<BTreeSet<SignalAttribute>, SetupError> {
  let mut extracted_attrs = BTreeSet::new();
  for attr in attrs.iter() {
    if !attr.path().is_ident("derive") {
      continue;
    }
    attr.parse_nested_meta(|meta| {
      let Some(last_segment) = meta.path.segments.last() else {
        return Err(syn::Error::new(
          meta.path.span(),
          "Missing derive item name",
        ));
      };
      let ident: &str = &last_segment.ident.to_string();
      let signal_attr_op = match ident {
        "SignalPiece" => Some(SignalAttribute::SignalPiece),
        "DartSignal" => Some(SignalAttribute::DartSignal),
        "DartSignalBinary" => Some(SignalAttribute::DartSignalBinary),
        "RustSignal" => Some(SignalAttribute::RustSignalBinary),
        "RustSignalBinary" => Some(SignalAttribute::RustSignalBinary),
        _ => None,
      };
      if let Some(signal_attr) = signal_attr_op {
        extracted_attrs.insert(signal_attr);
      }
      Ok(())
    })?;
  }
  Ok(extracted_attrs)
}

/// Convert a `syn` field type to a `serde_reflection::Format`.
/// This function handles common primitives
/// and container types like `Option` and `Vec`.
/// For unrecognized types, it returns a `TypeName`
/// with the type's string representation.
fn to_type_format(ty: &Type) -> Format {
  match ty {
    Type::Path(TypePath { path, .. }) => {
      // Get last segment
      // (e.g., for `std::collections::BTreeMap`, get `BTreeMap`).
      if let Some(last_segment) = path.segments.last() {
        let ident = last_segment.ident.to_string();

        match ident.as_str() {
          "u8" => Format::U8,
          "u16" => Format::U16,
          "u32" => Format::U32,
          "u64" => Format::U64,
          "u128" => Format::U128,
          "i8" => Format::I8,
          "i16" => Format::I16,
          "i32" => Format::I32,
          "i64" => Format::I64,
          "i128" => Format::I128,
          "f32" => Format::F32,
          "f64" => Format::F64,
          "bool" => Format::Bool,
          "char" => Format::Char,
          "String" => Format::Str,
          "Box" => {
            if let Some(inner) = extract_generic(last_segment) {
              to_type_format(&inner)
            } else {
              Format::unknown()
            }
          }
          "Option" => {
            if let Some(inner) = extract_generic(last_segment) {
              Format::Option(Box::new(to_type_format(&inner)))
            } else {
              Format::unknown()
            }
          }
          "Vec" | "HashSet" | "BTreeSet" => {
            if let Some(inner) = extract_generic(last_segment) {
              Format::Seq(Box::new(to_type_format(&inner)))
            } else {
              Format::unknown()
            }
          }
          "HashMap" | "BTreeMap" => {
            let generics = extract_generics(last_segment);
            if generics.len() == 2 {
              let key = to_type_format(&generics[0].to_owned());
              let value = to_type_format(&generics[1].to_owned());
              Format::Map {
                key: Box::new(key),
                value: Box::new(value),
              }
            } else {
              Format::unknown()
            }
          }
          _ => Format::TypeName(ident),
        }
      } else {
        Format::unknown()
      }
    }
    Type::Tuple(TypeTuple { elems, .. }) => {
      let formats: Vec<_> = elems.iter().map(to_type_format).collect();
      if formats.is_empty() {
        Format::Unit
      } else if formats.len() == 1 {
        formats[0].to_owned()
      } else {
        Format::Tuple(formats)
      }
    }
    Type::Array(TypeArray { elem, len, .. }) => {
      if let Expr::Lit(expr_lit) = len {
        if let Lit::Int(lit_int) = &expr_lit.lit {
          if let Ok(size) = lit_int.base10_parse::<usize>() {
            return Format::TupleArray {
              content: Box::new(to_type_format(elem)),
              size,
            };
          }
        }
      }
      Format::unknown()
    }
    _ => Format::unknown(),
  }
}

/// Extracts the first generic type argument
/// from a `PathSegment`, if available.
fn extract_generic(segment: &syn::PathSegment) -> Option<Type> {
  if let PathArguments::AngleBracketed(args) = &segment.arguments {
    args.args.iter().find_map(|arg| {
      if let GenericArgument::Type(ty) = arg {
        Some(ty.clone())
      } else {
        None
      }
    })
  } else {
    None
  }
}

/// Extracts all generic type arguments from a `PathSegment`.
fn extract_generics(segment: &syn::PathSegment) -> Vec<Type> {
  if let PathArguments::AngleBracketed(args) = &segment.arguments {
    args
      .args
      .iter()
      .filter_map(|arg| {
        if let GenericArgument::Type(ty) = arg {
          Some(ty.clone())
        } else {
          None
        }
      })
      .collect()
  } else {
    Vec::new()
  }
}

/// Trace a struct by collecting its field names (and a placeholder type)
/// and record its container format in the registry.
fn trace_struct(registry: &mut Registry, item: &ItemStruct) {
  // Collect basic information about this struct.
  let type_name = item.ident.to_string();

  // Collect the information about the container.
  let container = match &item.fields {
    syn::Fields::Unit => ContainerFormat::UnitStruct,
    syn::Fields::Unnamed(unnamed) => {
      let fields: Vec<Format> = unnamed
        .unnamed
        .iter()
        .map(|field| to_type_format(&field.ty))
        .collect();
      if fields.is_empty() {
        ContainerFormat::UnitStruct
      } else if fields.len() == 1 {
        ContainerFormat::NewTypeStruct(Box::new(fields[0].to_owned()))
      } else {
        ContainerFormat::TupleStruct(fields)
      }
    }
    syn::Fields::Named(named) => {
      let fields = named
        .named
        .iter()
        .filter_map(|field| {
          field.ident.as_ref().map(|ident| Named {
            name: ident.to_string(),
            value: to_type_format(&field.ty),
          })
        })
        .collect();
      ContainerFormat::Struct(fields)
    }
  };

  // Save the information about the container.
  registry.insert(type_name, container);
}

/// Trace an enum by collecting its variant names (and a placeholder type)
/// and record its container format in the registry.
fn trace_enum(registry: &mut Registry, item: &ItemEnum) {
  // Collect basic information about this enum.
  let type_name = item.ident.to_string();

  // Collect the information about the container.
  let variants: BTreeMap<u32, Named<VariantFormat>> = item
    .variants
    .iter()
    .map(|variant| {
      let name = variant.ident.to_string();
      let variant_format = match &variant.fields {
        syn::Fields::Unit => VariantFormat::Unit,
        syn::Fields::Unnamed(unnamed) => {
          let fields = unnamed
            .unnamed
            .iter()
            .map(|field| to_type_format(&field.ty))
            .collect::<Vec<_>>();
          if fields.is_empty() {
            VariantFormat::Unit
          } else if fields.len() == 1 {
            VariantFormat::NewType(Box::new(fields[0].to_owned()))
          } else {
            VariantFormat::Tuple(fields)
          }
        }
        syn::Fields::Named(named) => {
          let fields = named
            .named
            .iter()
            .filter_map(|field| {
              field.ident.as_ref().map(|ident| Named {
                name: ident.to_string(),
                value: to_type_format(&field.ty),
              })
            })
            .collect::<Vec<_>>();
          VariantFormat::Struct(fields)
        }
      };
      Named {
        name,
        value: variant_format,
      }
    })
    .enumerate()
    .map(|(index, value)| (index as u32, value))
    .collect();

  let container = ContainerFormat::Enum(variants);

  // Save the information about the container.
  registry.insert(type_name, container);
}

/// Process AST items and record struct types in the registry.
fn process_items(
  items: &[Item],
  registry: &mut Registry,
  signal_attrs: &mut BTreeMap<String, BTreeSet<SignalAttribute>>,
) -> Result<(), SetupError> {
  for item in items {
    match item {
      Item::Mod(m) if m.content.is_some() => {
        // Recursively process items in nested modules.
        if let Some(inner_items) = m.content.as_ref().map(|p| &p.1) {
          process_items(inner_items, registry, signal_attrs)?;
        }
      }
      Item::Struct(s) => {
        let extracted_attrs = extract_signal_attribute(&s.attrs)?;
        if !extracted_attrs.is_empty() {
          trace_struct(registry, s);
          signal_attrs.insert(s.ident.to_string(), extracted_attrs);
        }
      }
      Item::Enum(e) => {
        let extracted_attrs = extract_signal_attribute(&e.attrs)?;
        if !extracted_attrs.is_empty() {
          trace_enum(registry, e);
          signal_attrs.insert(e.ident.to_string(), extracted_attrs);
        }
      }
      _ => {}
    }
  }
  Ok(())
}

// TODO: Warn overlapping type names
// TODO: Disallow type names that starts with "Rinf"

fn visit_rust_files(
  dir: PathBuf,
  registry: &mut Registry,
  signal_attrs: &mut BTreeMap<String, BTreeSet<SignalAttribute>>,
) -> Result<(), SetupError> {
  let entries = read_dir(dir)?;
  for entry in entries.filter_map(Result::ok) {
    let path = entry.path();
    if path.is_dir() {
      // Recurse into subdirectory.
      visit_rust_files(path, registry, signal_attrs)?;
    } else {
      let content = read_to_string(path)?;
      let syntax_tree: File = syn::parse_file(&content)?;
      process_items(&syntax_tree.items, registry, signal_attrs)?;
    }
  }
  Ok(())
}

fn generate_class_extension_code(
  gen_dir: &Path,
  class: &str,
  extracted_attrs: &BTreeSet<SignalAttribute>,
) -> Result<(), SetupError> {
  let snake_class = class.to_case(Case::Snake);
  let class_file = gen_dir
    .join("generated")
    .join(format!("{}.dart", snake_class));
  let mut code = read_to_string(&class_file)?;

  if extracted_attrs.contains(&SignalAttribute::DartSignalBinary) {
    let new_code = format!(
      r#"
extension {class}DartSignalExt on {class} {{
  /// Sends the signal to Rust with separate binary data.
  /// Passing data from Rust to Dart involves a memory copy
  /// because Rust cannot own data managed by Dart's garbage collector.
  void sendSignalToRust(Uint8List binary) {{
    final messageBytes = bincodeSerialize();
      sendDartSignal(
        'rinf_send_dart_signal_{snake_class}',
        messageBytes,
        binary,
      );
    }}
  }}
}}
"#
    );
    code.push_str(&new_code);
  } else if extracted_attrs.contains(&SignalAttribute::DartSignal) {
    let new_code = format!(
      r#"
extension {class}DartSignalExt on {class} {{
  /// Sends the signal to Rust.
  /// Passing data from Rust to Dart involves a memory copy
  /// because Rust cannot own data managed by Dart's garbage collector.
  void sendSignalToRust() {{
    final messageBytes = bincodeSerialize();
    final binary = Uint8List(0);
    sendDartSignal(
      'rinf_send_dart_signal_{snake_class}',
      messageBytes,
      binary,
    );
  }}
}}
"#
    );
    code.push_str(&new_code);
  }

  write(&class_file, code)?;
  Ok(())
}

// TODO: Make some generated items private.

fn generate_class_interface_code(
  gen_dir: &Path,
  class: &str,
  extracted_attrs: &BTreeSet<SignalAttribute>,
) -> Result<(), SetupError> {
  let snake_class = class.to_case(Case::Snake);
  let class_file = gen_dir
    .join("generated")
    .join(format!("{}.dart", snake_class));
  let mut code = read_to_string(&class_file)?;

  let has_rust_signal = extracted_attrs.contains(&SignalAttribute::RustSignal)
    || extracted_attrs.contains(&SignalAttribute::RustSignalBinary);
  if has_rust_signal {
    let camel_class = class.to_case(Case::Camel);
    let new_code = format!(
      r#"
final {camel_class}StreamController =
    StreamController<RustSignalPack<{class}>>();
"#
    );
    code.push_str(&new_code);
    code = code.replacen(
      &format!("class {class} {{"),
      &format!(
        r#"class {class} {{
  /// An async broadcast stream that listens for signals from Rust.
  /// It supports multiple subscriptions.
  /// Make sure to cancel the subscription when it's no longer needed,
  /// such as when a widget is disposed.
  static final rustSignalStream =
      {camel_class}StreamController.stream.asBroadcastStream();
"#
      ),
      1,
    );
  }

  write(&class_file, code)?;
  Ok(())
}

fn generate_shared_code(
  gen_dir: &Path,
  signal_attrs: &BTreeMap<String, BTreeSet<SignalAttribute>>,
) -> Result<(), SetupError> {
  // Write type aliases.
  let mut code = r#"part of 'generated.dart';
"#
  .to_owned();

  // Write signal handler.
  code.push_str(
    "\nfinal assignRustSignal = \
        <String, void Function(Uint8List, Uint8List)>{",
  );
  for (class, extracted_attrs) in signal_attrs {
    let has_rust_signal = extracted_attrs
      .contains(&SignalAttribute::RustSignal)
      || extracted_attrs.contains(&SignalAttribute::RustSignalBinary);
    if !has_rust_signal {
      continue;
    }
    let camel_class = class.to_case(Case::Camel);
    let new_code = format!(
      r#"
  '{class}': (Uint8List messageBytes, Uint8List binary) {{
    final message = {class}.bincodeDeserialize(messageBytes);
    final rustSignal = RustSignalPack(
      message,
      binary,
    );
    {camel_class}StreamController.add(rustSignal);
  }},"#
    );
    code.push_str(&new_code);
  }
  code.push_str("\n};\n");

  // Save to a file.
  let shared_file = gen_dir.join("generated").join("signal_handlers.dart");
  write(&shared_file, code)?;
  Ok(())
}

fn generate_interface_code(
  gen_dir: &Path,
  signal_attrs: &BTreeMap<String, BTreeSet<SignalAttribute>>,
) -> Result<(), SetupError> {
  // Generate FFI interface code.
  for (class, extracted_attrs) in signal_attrs {
    generate_class_extension_code(gen_dir, class, extracted_attrs)?;
    generate_class_interface_code(gen_dir, class, extracted_attrs)?;
  }

  // Write imports.
  let top_file = gen_dir.join("generated").join("generated.dart");
  let mut top_content = read_to_string(&top_file)?;
  top_content = top_content.replacen(
    "export '../serde/serde.dart';",
    r#"import 'dart:async';
import 'package:rinf/rinf.dart';

export '../serde/serde.dart';"#,
    1,
  );
  top_content.push_str("part 'signal_handlers.dart';\n");
  write(&top_file, top_content)?;

  // Write the shared code.
  generate_shared_code(gen_dir, signal_attrs)?;
  Ok(())
}

pub fn generate_dart_code(
  root_dir: &Path,
  rinf_config: &RinfConfig,
) -> Result<(), SetupError> {
  // Analyze the input Rust files and collect type registries.
  let mut registry: Registry = Registry::new();
  let mut signal_attrs = BTreeMap::<String, BTreeSet<SignalAttribute>>::new();
  for crate_name in &rinf_config.gen_input_crates {
    let source_dir = root_dir.join("native").join(crate_name).join("src");
    visit_rust_files(source_dir, &mut registry, &mut signal_attrs)?;
  }

  // TODO: Include comments from original structs with `with_comments` method
  // TODO: Warn properly when Rust syntax is invalid

  // Empty the generation folder.
  let gen_dir = root_dir.join(rinf_config.gen_output_dir.clone());
  let _ = remove_dir_all(&gen_dir);
  create_dir_all(&gen_dir)?;

  // Create the code generator config.
  let gen_config = CodeGeneratorConfig::new("generated".to_string())
    .with_encodings([Encoding::Bincode])
    .with_package_manifest(false)
    .with_c_style_enums(true);

  // Install serialization modules.
  let installer = Installer::new(gen_dir.clone());
  installer
    .install_module(&gen_config, &registry)
    .map_err(|_| SetupError::ReflectionModule)?;
  installer
    .install_serde_runtime()
    .map_err(|_| SetupError::ReflectionModule)?;
  installer
    .install_bincode_runtime()
    .map_err(|_| SetupError::ReflectionModule)?;

  // Generate Dart serialization code from the registry.
  let generator = CodeGenerator::new(&gen_config);
  generator.output(gen_dir.clone(), &registry)?;
  move_directory_contents(&gen_dir.join("lib").join("src"), &gen_dir)?;
  remove_dir_all(gen_dir.join("lib"))?;

  // Write the export file.
  let gen_dir_name = gen_dir
    .file_name()
    .and_then(|s| s.to_str())
    .unwrap_or("bindings");
  write(
    gen_dir.join(format!("{gen_dir_name}.dart")),
    "export 'generated/generated.dart';",
  )?;

  // Generate Dart interface code for FFI.
  generate_interface_code(&gen_dir, &signal_attrs)?;
  Ok(())
}

// TODO: Clean up CLI output
// TODO: `watch_and_generate_dart_code` is not tested, so check it later

/// Watches the Rust source directory for changes and regenerates Dart code.
pub fn watch_and_generate_dart_code(
  root_dir: &Path,
  message_config: &RinfConfig,
) -> Result<(), SetupError> {
  // Prepare the source directory for Rust files.
  let source_dir = root_dir.join("native").join("hub").join("src");
  if !source_dir.exists() {
    Err(SetupError::ProjectStructure(
      "Source directory of the `hub` crate does not exist",
    ))?;
  }

  // Create a channel to receive file change events.
  let (sender, receiver) = channel();

  // Create a file system watcher using the new notify API.
  let mut watcher = RecommendedWatcher::new(
    move |res: Result<Event, notify::Error>| {
      // Send events to the channel.
      let result = sender.send(res);
      if let Err(err) = result {
        eprintln!("{err}");
      }
    },
    Config::default(),
  )?;

  // Start watching the source directory recursively.
  watcher.watch(&source_dir, RecursiveMode::Recursive)?;

  loop {
    // Block until an event is received.
    match receiver.recv() {
      Ok(Ok(event)) => {
        if should_regenerate(&event) {
          eprintln!("File change detected: {:?}", event);
          let result = generate_dart_code(root_dir, message_config);
          if let Err(err) = result {
            eprintln!("{err}");
          }
        }
      }
      Ok(Err(e)) => {
        eprintln!("Watch error: {:?}", e);
        break;
      }
      Err(e) => {
        eprintln!("Channel receive error: {:?}", e);
        break;
      }
    }

    // Optional: sleep briefly to avoid busy looping (if necessary).
    std::thread::sleep(Duration::from_millis(100));
  }
  Ok(())
}

/// Determines whether the event requires
/// regenerating Dart code by checking if any changed file is a Rust source.
fn should_regenerate(event: &Event) -> bool {
  event
    .paths
    .iter()
    .any(|path| path.extension().map(|ext| ext == "rs").unwrap_or(false))
}

/// Iterate over the files and directories in A.
/// Then, move each inner file or directory to B.
/// This function is recursive and checks all nested children.
fn move_directory_contents(
  dir_from: &Path,
  dir_to: &Path,
) -> Result<(), SetupError> {
  if !dir_to.is_dir() {
    create_dir_all(dir_to)?;
  }
  for entry_result in read_dir(dir_from)? {
    let entry = entry_result?;
    let src_path = entry.path();
    let dest_path = dir_to.join(entry.file_name());
    if src_path.is_dir() {
      move_directory_contents(&src_path, &dest_path)?;
    } else {
      rename(&src_path, &dest_path)?;
    }
  }
  Ok(())
}
