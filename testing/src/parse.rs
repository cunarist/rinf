use quote::quote;
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

/// Trace a struct by collecting its field names (and a placeholder type)
/// and record its container format in the registry.
fn trace_struct(registry: &mut Registry, s: &ItemStruct) {
    let mut fields = Vec::new();
    for field in s.fields.iter() {
        if let Some(ident) = &field.ident {
            // Use the `quote!` macro to extract the field type as a string (for logging).
            let ty_string = quote! { #field.ty }.to_string();
            println!("    Tracing Field: {} with type: {}", ident, ty_string);
            // For now, we use Format::Str as a placeholder for the field's type.
            fields.push(Named {
                name: ident.to_string(),
                value: Format::Str,
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
    println!("------\nRegistry from `Copy` structs:\n------");
    for (type_name, container) in &registry {
        println!("Type `{}`", type_name);
        println!("{:?}", container);
        println!("------");
    }
}
