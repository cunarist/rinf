use crate::{RinfConfigMessage, SetupError};
use convert_case::{Case, Casing};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use quote::ToTokens;
use serde_generate::dart::{CodeGenerator, Installer};
use serde_generate::{CodeGeneratorConfig, Encoding, SourceInstaller};
use serde_reflection::{ContainerFormat, Format, Named, Registry};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{read_dir, read_to_string, remove_file, write};
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;
use syn::spanned::Spanned;
use syn::{
    Attribute, Expr, ExprLit, File, GenericArgument, Item, ItemStruct, Lit,
    PathArguments, Type, TypeArray, TypePath, TypeTuple,
};

// TODO: Handle enums and tuple structs

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
                    "Option" => {
                        if let Some(inner) = extract_generic(last_segment) {
                            Format::Option(Box::new(to_type_format(&inner)))
                        } else {
                            Format::TypeName("Option<?>".to_string())
                        }
                    }
                    "Vec" => {
                        if let Some(inner) = extract_generic(last_segment) {
                            Format::Seq(Box::new(to_type_format(&inner)))
                        } else {
                            Format::TypeName("Vec<?>".to_string())
                        }
                    }
                    "BTreeMap" => {
                        let mut generics = extract_generics(last_segment);
                        if generics.len() == 2 {
                            let key = to_type_format(&generics.remove(0));
                            let value = to_type_format(&generics.remove(0));
                            Format::Map {
                                key: Box::new(key),
                                value: Box::new(value),
                            }
                        } else {
                            Format::TypeName("BTreeMap<?, ?>".to_string())
                        }
                    }
                    _ => Format::TypeName(ident),
                }
            } else {
                Format::TypeName(ty.to_token_stream().to_string())
            }
        }
        Type::Tuple(TypeTuple { elems, .. }) => {
            let formats: Vec<_> = elems.iter().map(to_type_format).collect();
            Format::Tuple(formats)
        }
        Type::Array(TypeArray { elem, len, .. }) => {
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(lit_int),
                ..
            }) = len
            {
                if let Ok(size) = lit_int.base10_parse::<usize>() {
                    return Format::TupleArray {
                        content: Box::new(to_type_format(elem)),
                        size,
                    };
                }
            }
            Format::TypeName(ty.to_token_stream().to_string())
        }
        _ => Format::TypeName(ty.to_token_stream().to_string()),
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
        args.args
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
        vec![]
    }
}

/// Trace a struct by collecting its field names (and a placeholder type)
/// and record its container format in the registry.
fn trace_struct(registry: &mut Registry, s: &ItemStruct) {
    let mut fields = Vec::new();
    for field in s.fields.iter() {
        if let Some(ident) = &field.ident {
            let field_format = to_type_format(&field.ty);
            fields.push(Named {
                name: ident.to_string(),
                value: field_format,
            });
        }
    }

    // Build the container format for the struct.
    let container = ContainerFormat::Struct(fields);

    // Insert the struct's container format
    // into the registry using its identifier as key.
    let type_name = s.ident.to_string();
    registry.insert(type_name, container);
}

/// Process AST items and record struct types in the registry.
fn process_items(
    items: &[Item],
    registry: &mut Registry,
    signal_attrs: &mut BTreeMap<String, BTreeSet<SignalAttribute>>,
) -> Result<(), SetupError> {
    let mut structs = Vec::new();
    for item in items {
        match item {
            Item::Struct(s) => {
                let extracted_attrs = extract_signal_attribute(&s.attrs)?;
                if !extracted_attrs.is_empty() {
                    structs.push(s.ident.clone());
                    trace_struct(registry, s);
                    signal_attrs.insert(s.ident.to_string(), extracted_attrs);
                }
            }
            Item::Mod(m) if m.content.is_some() => {
                // Recursively process items in nested modules.
                if let Some(inner_items) = m.content.as_ref().map(|p| &p.1) {
                    process_items(inner_items, registry, signal_attrs)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

// TODO: Warn overlapping type names

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
    root_dir: &Path,
    class: &str,
    extracted_attrs: &BTreeSet<SignalAttribute>,
) -> Result<(), SetupError> {
    let snake_class = class.to_case(Case::Snake);
    let class_file = root_dir
        .join("lib")
        .join("src")
        .join("generated")
        .join(format!("{}.dart", snake_class));
    let mut code = read_to_string(&class_file)?;

    if extracted_attrs.contains(&SignalAttribute::DartSignalBinary) {
        let new_code = format!(
            r#"
extension {class}DartSignalExt on {class} {{
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
    root_dir: &Path,
    class: &str,
    extracted_attrs: &BTreeSet<SignalAttribute>,
) -> Result<(), SetupError> {
    let snake_class = class.to_case(Case::Snake);
    let class_file = root_dir
        .join("lib")
        .join("src")
        .join("generated")
        .join(format!("{}.dart", snake_class));
    let mut code = read_to_string(&class_file)?;

    let has_rust_signal = extracted_attrs
        .contains(&SignalAttribute::RustSignal)
        || extracted_attrs.contains(&SignalAttribute::RustSignalBinary);
    if has_rust_signal {
        let camel_class = class.to_case(Case::Camel);
        let new_code = format!(
            r#"
final {camel_class}StreamController =
    StreamController<RustSignal<{class}>>();
"#
        );
        code.push_str(&new_code);
        code = code.replacen(
            &format!("class {class} {{"),
            &format!(
                r#"class {class} {{
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

// TODO: Delete the folder before generating

fn generate_shared_code(
    root_dir: &Path,
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
    final rustSignal = RustSignal(
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
    let shared_file = root_dir
        .join("lib")
        .join("src")
        .join("generated")
        .join("signal_handlers.dart");
    write(&shared_file, code)?;
    Ok(())
}

fn generate_interface_code(
    root_dir: &Path,
    signal_attrs: &BTreeMap<String, BTreeSet<SignalAttribute>>,
) -> Result<(), SetupError> {
    // Generate FFI interface code.
    for (class, extracted_attrs) in signal_attrs {
        generate_class_extension_code(root_dir, class, extracted_attrs)?;
        generate_class_interface_code(root_dir, class, extracted_attrs)?;
    }

    // Write imports.
    let top_file = root_dir
        .join("lib")
        .join("src")
        .join("generated")
        .join("generated.dart");
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
    generate_shared_code(root_dir, signal_attrs)?;
    Ok(())
}

pub fn generate_dart_code(
    root_dir: &Path,
    _message_config: &RinfConfigMessage,
) -> Result<(), SetupError> {
    // TODO: Use the config

    // Analyze the input Rust files and collect type registries.
    let mut registry: Registry = Registry::new();
    let mut signal_attrs = BTreeMap::<String, BTreeSet<SignalAttribute>>::new();
    let source_dir = root_dir.join("native").join("hub").join("src");
    visit_rust_files(source_dir, &mut registry, &mut signal_attrs)?;

    // TODO: Include comments from original structs with `with_comments` method

    // Create the code generator config.
    let config = CodeGeneratorConfig::new("generated".to_string())
        .with_encodings([Encoding::Bincode])
        .with_package_manifest(false);

    // Install serialization modules.
    let installer = Installer::new(root_dir.to_owned());
    installer
        .install_module(&config, &registry)
        .map_err(|_| SetupError::ReflectionModule)?;
    installer
        .install_serde_runtime()
        .map_err(|_| SetupError::ReflectionModule)?;
    installer
        .install_bincode_runtime()
        .map_err(|_| SetupError::ReflectionModule)?;

    // Generate Dart serialization code from the registry.
    let generator = CodeGenerator::new(&config);
    generator.output(root_dir.to_owned(), &registry)?;

    // Generate Dart interface code.
    generate_interface_code(root_dir, &signal_attrs)?;

    // Remove unnecessary files.
    remove_file(root_dir.join("lib").join("generated.dart"))?;
    Ok(())
}

// TODO: `watch_and_generate_dart_code` is not tested, so check it later

/// Watches the Rust source directory for changes and regenerates Dart code.
pub fn watch_and_generate_dart_code(
    root_dir: &Path,
    message_config: &RinfConfigMessage,
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
            if let Err(error) = result {
                eprintln!("{}", error);
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
                    if let Err(error) = result {
                        eprintln!("{}", error);
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
