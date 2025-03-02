use quote::ToTokens;
use serde_generate::dart::{CodeGenerator, Installer};
use serde_generate::{CodeGeneratorConfig, Encoding, SourceInstaller};
use serde_reflection::{ContainerFormat, Format, Named, Registry};
use std::env::current_dir;
use std::fs;
use std::path::PathBuf;
use syn::{
    Attribute, Expr, ExprLit, File, GenericArgument, Item, ItemStruct, Lit,
    PathArguments, Type, TypeArray, TypePath, TypeTuple,
};

// TODO: Remove all panicking code.

fn implements_signal(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if attr.path().is_ident("derive") {
            println!("Found #[derive(...)]");

            let mut found = false;
            let _ = attr.parse_nested_meta(|meta| {
                println!("Checking meta: {:?}", meta.path.get_ident());

                let last_segment = meta.path.segments.last().unwrap();
                let ident: &str = &last_segment.ident.to_string();
                found = matches!(ident, "Signal" | "DartSignal" | "RustSignal");
                Ok(())
            });

            found
        } else {
            println!("Not a derive attribute");
            false
        }
    })
}

/// Convert a `syn` field type to a `serde_reflection::Format`.
/// This function handles common primitives
/// and container types like `Option` and `Vec``.
/// For unrecognized types, it returns a `TypeName`
/// with the type's string representation.
fn to_type_format(ty: &Type) -> Format {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            // Get last segment
            // (e.g., for `std::collections::HashMap`, get `HashMap`).
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
                    "HashMap" => {
                        let mut generics = extract_generics(last_segment);
                        if generics.len() == 2 {
                            let key = to_type_format(&generics.remove(0));
                            let value = to_type_format(&generics.remove(0));
                            Format::Map {
                                key: Box::new(key),
                                value: Box::new(value),
                            }
                        } else {
                            Format::TypeName("HashMap<?, ?>".to_string())
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
                lit: Lit::Int(ref lit_int),
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
            println!(
                "    Tracing Field: {} with type format: {:?}",
                ident, field_format
            );
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
fn process_items(registry: &mut Registry, items: &[Item]) {
    let mut structs = Vec::new();
    for item in items {
        match item {
            Item::Struct(s) if implements_signal(&s.attrs) => {
                trace_struct(registry, s);
                structs.push(s.ident.clone());
            }
            Item::Mod(m) if m.content.is_some() => {
                // Recursively process items in nested modules.
                process_items(registry, &m.content.as_ref().unwrap().1);
            }
            _ => {}
        }
    }
}

fn visit_rust_files(dir: PathBuf, registry: &mut Registry) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                visit_rust_files(path, registry); // Recurse into subdirectory
            } else {
                let content = fs::read_to_string(path).unwrap();
                let syntax_tree: File = syn::parse_file(&content)
                    .expect("Failed to parse Rust file");
                process_items(registry, &syntax_tree.items);
            }
        }
    }
}

pub fn generate_dart_code() {
    // TODO: Check if the CWD has correct `pubspec.yaml` file.

    // Prepare paths.
    let current_path = current_dir().unwrap();
    let pubspec_path = current_path.join("pubspec.yaml");

    // Read the file.
    // TODO: Remove
    let pubpsec_content = fs::read_to_string(&pubspec_path).unwrap();

    // Analyze the input Rust files
    // and collect type registries.
    let mut registry: Registry = Registry::new();
    let source_dir = current_path.join("native").join("hub").join("src");
    visit_rust_files(source_dir, &mut registry);

    // Create the code generator config.
    let config = CodeGeneratorConfig::new("generated".to_string())
        .with_encodings([Encoding::Bincode]);

    // Install serialization modules.
    let installer = Installer::new(PathBuf::from("./"));
    installer.install_module(&config, &registry).unwrap();
    installer.install_serde_runtime().unwrap();
    installer.install_bincode_runtime().unwrap();

    // Generate Dart code from the registry.
    // Create the Dart code generator.
    let generator = CodeGenerator::new(&config);
    generator.output(current_path, &registry).unwrap();

    // Write back to pubspec file.
    // TODO: Remove
    fs::write(&pubspec_path, pubpsec_content).unwrap();

    // Write the generated Dart code to the output file.
    println!("Dart code generated!");
}
