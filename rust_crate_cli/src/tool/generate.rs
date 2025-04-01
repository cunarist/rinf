use crate::dimmedln;
use crate::tool::{CleanFileName, RinfConfig, SetupError};
use convert_case::{Case, Casing};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde_generate::dart::{CodeGenerator, Installer};
use serde_generate::{CodeGeneratorConfig, Encoding, SourceInstaller};
use serde_reflection::{ContainerFormat, Format, Named, VariantFormat};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{
  create_dir_all, read_dir, read_to_string, remove_dir_all, rename, write,
};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use syn::spanned::Spanned;
use syn::{
  Attribute, Expr, File, GenericArgument, Item, ItemEnum, ItemStruct, Lit,
  PathArguments, Type, TypeArray, TypePath, TypeTuple,
};

static GEN_MOD: &str = "signals";

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum SignalAttribute {
  SignalPiece,
  DartSignal,
  DartSignalBinary,
  RustSignal,
  RustSignalBinary,
}

fn extract_signal_attributes(
  attrs: &[Attribute],
) -> Option<BTreeSet<SignalAttribute>> {
  let mut extracted_attrs = BTreeSet::new();
  for attr in attrs.iter() {
    if !attr.path().is_ident("derive") {
      continue;
    }
    attr
      .parse_nested_meta(|meta| {
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
      })
      .ok()?;
  }
  Some(extracted_attrs)
}

fn extract_doc_comment(attrs: &[Attribute]) -> String {
  let lines: Vec<String> = attrs
    .iter()
    .filter_map(|attr| {
      // Check if the attribute is a doc comment
      if attr.path().is_ident("doc") {
        // Parse the attribute as a `MetaNameValue`
        if let syn::Meta::NameValue(meta) = &attr.meta {
          if let syn::Expr::Lit(lit) = &meta.value {
            if let syn::Lit::Str(lit_str) = &lit.lit {
              return Some(lit_str.value().trim().to_owned());
            }
          }
        }
      }
      None
    })
    .collect();
  lines.join("\n")
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
fn trace_struct(traced: &mut Traced, item: &ItemStruct) {
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
  traced.registry.insert(type_name, container);
}

/// Trace an enum by collecting its variant names (and a placeholder type)
/// and record its container format in the registry.
fn trace_enum(traced: &mut Traced, item: &ItemEnum) {
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
  traced.registry.insert(type_name, container);
}

/// Checks that the name of newly found signal is usable.
fn check_signal_name(name: &str, traced: &Traced) -> Result<(), SetupError> {
  if traced.registry.contains_key(name) {
    Err(SetupError::DuplicatedSignal(name.to_owned()))?
  }
  Ok(())
}

/// Process AST items and record struct types in the registry.
fn process_items_in_module(
  items: &[Item],
  traced: &mut Traced,
  file_name: &str,
) -> Result<(), SetupError> {
  for item in items {
    match item {
      Item::Mod(m) if m.content.is_some() => {
        // Recursively process items in nested modules.
        if let Some(inner_items) = m.content.as_ref().map(|p| &p.1) {
          process_items_in_module(inner_items, traced, file_name)?;
        }
      }
      Item::Struct(s) => {
        let item_name = s.ident.to_string();
        check_signal_name(&item_name, traced)?;
        let signal_attrs = extract_signal_attributes(&s.attrs)
          .ok_or(SetupError::CodeSyntax(file_name.to_owned()))?;
        if !signal_attrs.is_empty() {
          trace_struct(traced, s);
          traced.signal_attrs.insert(item_name.clone(), signal_attrs);
          let doc_comment = extract_doc_comment(&s.attrs);
          let item_path = vec![GEN_MOD.to_owned(), item_name];
          traced.doc_comments.insert(item_path, doc_comment);
        }
      }
      Item::Enum(e) => {
        let item_name = e.ident.to_string();
        check_signal_name(&item_name, traced)?;
        let signal_attrs = extract_signal_attributes(&e.attrs)
          .ok_or(SetupError::CodeSyntax(file_name.to_owned()))?;
        if !signal_attrs.is_empty() {
          trace_enum(traced, e);
          traced.signal_attrs.insert(item_name.clone(), signal_attrs);
          let doc_comment = extract_doc_comment(&e.attrs);
          let item_path = vec![GEN_MOD.to_owned(), item_name];
          traced.doc_comments.insert(item_path, doc_comment);
        }
      }
      _ => {}
    }
  }
  Ok(())
}

struct Traced {
  registry: BTreeMap<String, ContainerFormat>,
  signal_attrs: BTreeMap<String, BTreeSet<SignalAttribute>>,
  doc_comments: BTreeMap<Vec<String>, String>,
}

fn visit_rust_files(dir: &Path, traced: &mut Traced) -> Result<(), SetupError> {
  let entries = read_dir(dir)?;
  for entry in entries.filter_map(Result::ok) {
    let entry_path = entry.path();
    let file_name = entry_path.clean_file_name()?;
    if entry_path.is_dir() {
      // Recurse into subdirectory.
      visit_rust_files(&entry_path, traced)?;
    } else {
      let content = read_to_string(&entry_path)?;
      let syntax_tree: File = syn::parse_file(&content)
        .map_err(|_| SetupError::CodeSyntax(file_name.to_owned()))?;
      process_items_in_module(&syntax_tree.items, traced, &file_name)?;
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
  let class_file = gen_dir.join(GEN_MOD).join(format!("{}.dart", snake_class));
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

fn generate_class_interface_code(
  gen_dir: &Path,
  class: &str,
  extracted_attrs: &BTreeSet<SignalAttribute>,
) -> Result<(), SetupError> {
  let snake_class = class.to_case(Case::Snake);
  let class_file = gen_dir.join(GEN_MOD).join(format!("{}.dart", snake_class));
  let mut code = read_to_string(&class_file)?;

  let has_rust_signal = extracted_attrs.contains(&SignalAttribute::RustSignal)
    || extracted_attrs.contains(&SignalAttribute::RustSignalBinary);
  if has_rust_signal {
    let camel_class = class.to_case(Case::Camel);
    let new_code = format!(
      r#"
final _{camel_class}StreamController =
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
      _{camel_class}StreamController.stream.asBroadcastStream();
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
  let mut code = format!("part of '{}.dart';\n", GEN_MOD);

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
    _{camel_class}StreamController.add(rustSignal);
  }},"#
    );
    code.push_str(&new_code);
  }
  code.push_str("\n};\n");

  // Save to a file.
  let shared_file = gen_dir.join(GEN_MOD).join("signal_handlers.dart");
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
  let top_file = gen_dir.join(GEN_MOD).join(format!("{GEN_MOD}.dart"));
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
  let mut traced = Traced {
    registry: BTreeMap::new(),
    signal_attrs: BTreeMap::new(),
    doc_comments: BTreeMap::new(),
  };
  for crate_name in &rinf_config.gen_input_crates {
    let source_dir = root_dir.join("native").join(crate_name).join("src");
    visit_rust_files(&source_dir, &mut traced)?;
  }

  // Empty the generation folder.
  let gen_dir = root_dir.join(rinf_config.gen_output_dir.clone());
  let _ = remove_dir_all(&gen_dir);
  create_dir_all(&gen_dir)?;

  // Create the code generator config.
  let gen_config = CodeGeneratorConfig::new(GEN_MOD.to_string())
    .with_encodings([Encoding::Bincode])
    .with_package_manifest(false)
    .with_c_style_enums(true)
    .with_comments(traced.doc_comments);

  // Install serialization modules.
  let installer = Installer::new(gen_dir.clone());
  installer
    .install_module(&gen_config, &traced.registry)
    .map_err(|_| SetupError::ReflectionModule)?;
  installer
    .install_serde_runtime()
    .map_err(|_| SetupError::ReflectionModule)?;
  installer
    .install_bincode_runtime()
    .map_err(|_| SetupError::ReflectionModule)?;

  // Generate Dart class code from the registry.
  let generator = CodeGenerator::new(&gen_config);
  generator.output(gen_dir.clone(), &traced.registry)?;
  move_directory_contents(&gen_dir.join("lib").join("src"), &gen_dir)?;
  remove_dir_all(gen_dir.join("lib"))?;

  // Remove lint warnings from generated code
  remove_lint_warnings(&gen_dir)?;

  // Write the export file.
  let gen_dir_name = gen_dir.clean_file_name()?;
  write(
    gen_dir.join(format!("{gen_dir_name}.dart")),
    format!("export '{}/{}.dart';", GEN_MOD, GEN_MOD),
  )?;

  // Generate Dart interface code for FFI.
  generate_interface_code(&gen_dir, &traced.signal_attrs)?;
  Ok(())
}

/// Watches the Rust source directory for changes and regenerates Dart code.
pub fn watch_and_generate_dart_code(
  root_dir: &Path,
  rinf_config: &RinfConfig,
) -> Result<(), SetupError> {
  // Create a channel to pass file change events.
  let (sender, receiver) = channel();

  // Create file system watchers using the new notify API.
  dimmedln!("Watching Rust files");
  let mut watcher = RecommendedWatcher::new(
    move |event_result| {
      // Send events to the channel.
      let event = match event_result {
        Ok(inner) => inner,
        Err(err) => {
          eprintln!("Watch error: {}", err);
          return;
        }
      };
      let send_result = sender.send(event);
      if let Err(err) = send_result {
        eprintln!("{}", err);
      }
    },
    Config::default(),
  )?;
  for crate_name in &rinf_config.gen_input_crates {
    let source_dir = root_dir.join("native").join(crate_name);
    watcher.watch(&source_dir, RecursiveMode::Recursive)?;
  }

  loop {
    // Sleep briefly to avoid busy looping.
    std::thread::sleep(Duration::from_millis(100));
    // Block until an event is received.
    match receiver.recv() {
      Ok(event) => {
        if should_regenerate(&event) {
          dimmedln!(
            "Change detected in file `{}`",
            event.paths[0].clean_file_name()?
          );
          let result = generate_dart_code(root_dir, rinf_config);
          if let Err(err) = result {
            eprintln!("{}", err);
          }
        }
      }
      Err(err) => {
        eprintln!("{}", err);
        break;
      }
    }
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

fn remove_lint_warnings(gen_dir: &Path) -> Result<(), SetupError> {
  write_lint_ignore(&gen_dir.join("serde").join("serde.dart"))?;
  write_lint_ignore(&gen_dir.join("bincode").join("bincode.dart"))?;
  write_lint_ignore(&gen_dir.join(GEN_MOD).join(format!("{}.dart", GEN_MOD)))?;
  Ok(())
}

fn write_lint_ignore(file_path: &Path) -> Result<(), SetupError> {
  let content = read_to_string(file_path)?;
  write(
    file_path,
    format!("// ignore_for_file: type=lint, type=warning\n{}", content),
  )?;
  Ok(())
}
