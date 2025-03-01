use quote::ToTokens;
use serde_reflection::{ContainerFormat, Format, Named, Registry};
use std::fs;
use syn::{Attribute, File, Item, ItemStruct};

fn implements_copy(attrs: &[Attribute]) -> bool {
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
            if other.starts_with("Option<") || other.starts_with("Option <") {
                // Remove the prefix and suffix to get the inner type.
                let inner = other
                    .trim_start_matches("Option<")
                    .trim_start_matches("Option <")
                    .trim_end_matches('>');
                if let Ok(inner_ty) = syn::parse_str::<syn::Type>(inner) {
                    return Format::Option(Box::new(to_type_format(&inner_ty)));
                }
            }
            // Handle Vec<T> as a sequence.
            if other.starts_with("Vec<") || other.starts_with("Vec <") {
                let inner = other
                    .trim_start_matches("Vec<")
                    .trim_start_matches("Vec <")
                    .trim_end_matches('>');
                if let Ok(inner_ty) = syn::parse_str::<syn::Type>(inner) {
                    return Format::Seq(Box::new(to_type_format(&inner_ty)));
                }
            }
            // Fallback: return the type name.
            Format::TypeName(other.to_string())
        }
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

pub fn analyze_file() {
    let file_path =
        "../flutter_package/example/native/hub/src/messages/counter_number.rs";
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let syntax_tree: File =
        syn::parse_file(&content).expect("Failed to parse Rust file");

    let mut registry: Registry = Registry::new();
    process_items(&mut registry, &syntax_tree.items);
    println!("------\nRegistry from `Copy` types:\n------");
    for (type_name, container) in &registry {
        println!("Type `{}`", type_name);
        println!("{:?}", container);
        println!("------");
    }
}
