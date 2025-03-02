use quote::ToTokens;
use serde_generate::dart::{CodeGenerator, Installer};
use serde_generate::{CodeGeneratorConfig, Encoding, SourceInstaller};
use serde_reflection::{ContainerFormat, Format, Named, Registry};
use std::env::current_dir;
use std::fs;
use std::path::PathBuf;
use syn::{Attribute, File, Item, ItemStruct};

// TODO: Remove all panicking code.

fn implements_copy(attrs: &[Attribute]) -> bool {
    return true;
    attrs.iter().any(|attr| {
        if attr.path().is_ident("derive") {
            println!("Found #[derive(...)]");

            let mut found = false;
            let _ = attr.parse_nested_meta(|meta| {
                println!("Checking meta: {:?}", meta.path.get_ident());
                if meta.path.is_ident("Copy") {
                    found = true;
                }
                Ok(())
            });

            found
        } else {
            println!("Not a derive attribute");
            false
        }
    })
}

/// Convert a syn field type to a serde_reflection::Format.
/// This function handles common primitives and container types like Option and Vec.
/// For unrecognized types, it returns a TypeName with the type's string representation.
fn to_type_format(ty: &syn::Type) -> Format {
    // Get a string representation of the type.
    let ty_str = ty.to_token_stream().to_string();
    let trimmed = ty_str.trim();

    match trimmed {
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
        other => {
            // Handle Option<T>
            if other.starts_with("Option") {
                let inner = extract_inner_type(other, "Option");
                if let Ok(inner_ty) = syn::parse_str::<syn::Type>(inner) {
                    return Format::Option(Box::new(to_type_format(&inner_ty)));
                }
            }

            // Handle Vec<T> as a sequence.
            if other.starts_with("Vec") {
                let inner = extract_inner_type(other, "Vec");
                if let Ok(inner_ty) = syn::parse_str::<syn::Type>(inner) {
                    return Format::Seq(Box::new(to_type_format(&inner_ty)));
                }
            }

            // Handle HashMap<K, V> as a map.
            if other.starts_with("HashMap") {
                let inner = extract_inner_type(other, "HashMap");
                let parts: Vec<&str> =
                    inner.split(',').map(str::trim).collect();
                if parts.len() == 2 {
                    if let (Ok(k), Ok(v)) = (
                        syn::parse_str::<syn::Type>(parts[0]),
                        syn::parse_str::<syn::Type>(parts[1]),
                    ) {
                        return Format::Map {
                            key: Box::new(to_type_format(&k)),
                            value: Box::new(to_type_format(&v)),
                        };
                    }
                }
            }

            // Handle Tuples (e.g., "(Foo, Bar)")
            if other.starts_with('(') && other.ends_with(')') {
                let inner = other.trim_start_matches('(').trim_end_matches(')');
                let types: Vec<_> = inner
                    .split(',')
                    .map(str::trim)
                    .filter_map(|s| syn::parse_str::<syn::Type>(s).ok())
                    .collect();
                if !types.is_empty() {
                    return Format::Tuple(
                        types.iter().map(to_type_format).collect(),
                    );
                }
            }

            // Handle TupleArrays (e.g., "[Foo; N]")
            if other.starts_with('[') && other.ends_with(']') {
                let inner = other.trim_start_matches('[').trim_end_matches(']');
                let parts: Vec<&str> =
                    inner.split(';').map(str::trim).collect();
                if parts.len() == 2 {
                    if let (Ok(content_ty), Ok(size)) = (
                        syn::parse_str::<syn::Type>(parts[0]),
                        parts[1].parse::<usize>(),
                    ) {
                        return Format::TupleArray {
                            content: Box::new(to_type_format(&content_ty)),
                            size,
                        };
                    }
                }
            }

            // Fallback: return type name
            Format::TypeName(other.to_string())
        }
    }
}

/// Extracts the inner type
/// from a generic type like `Option<T>` or `Vec<T>`.
fn extract_inner_type<'a>(input: &'a str, prefix: &'a str) -> &'a str {
    input
        .trim_start_matches(prefix)
        .trim()
        .trim_start_matches('<')
        .trim_end_matches('>')
        .trim()
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

    // Insert the struct's container format into the registry using its identifier as key.
    let type_name = s.ident.to_string();
    registry.insert(type_name, container);
}

/// Process AST items and record struct types in the registry.
fn process_items(registry: &mut Registry, items: &[Item]) {
    let mut structs = Vec::new();
    for item in items {
        match item {
            Item::Struct(s) if implements_copy(&s.attrs) => {
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

pub fn generate_dart_code() {
    // TODO: Check if the CWD has correct `pubspec.yaml` file.

    // Prepare paths.
    let current_path = current_dir().unwrap();
    let pubspec_path = current_path.join("pubspec.yaml");

    // Read the file.
    // TODO: Remove
    let pubpsec_content = fs::read_to_string(&pubspec_path).unwrap();

    // Analyze the input Rust file.
    // TODO: Traverse all nested module files.
    let file_path = current_path
        .join("native")
        .join("hub")
        .join("src")
        .join("messages")
        .join("counter_number.rs");
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let syntax_tree: File =
        syn::parse_file(&content).expect("Failed to parse Rust file");

    // Collect type registries.
    let mut registry: Registry = Registry::new();
    process_items(&mut registry, &syntax_tree.items);
    println!("------\nRegistry from `Copy` types:\n------");
    for (type_name, container) in &registry {
        println!("Type `{}`", type_name);
        println!("{:?}", container);
        println!("------");
    }

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
