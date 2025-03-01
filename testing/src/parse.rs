use std::fs;
use syn::__private::ToTokens; // For test printing
use syn::{Attribute, File, Ident, Item};

fn implements_copy(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if attr.path().is_ident("derive") {
            println!("Found #[derive(...)]");

            let mut found = false;
            let _ = attr.parse_nested_meta(|meta| {
                println!("Checking meta: {}", meta.path.to_token_stream());
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

fn extract_copy_structs(file_path: &str) -> Vec<Ident> {
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let syntax_tree: File =
        syn::parse_file(&content).expect("Failed to parse Rust file");

    syntax_tree
        .items
        .into_iter()
        .filter_map(|item| {
            if let Item::Struct(s) = item {
                if implements_copy(&s.attrs) {
                    return Some(s.ident);
                }
            }
            None
        })
        .collect()
}

pub fn analyze_file() {
    let structs = extract_copy_structs(
        "../flutter_package/example/native/hub/src/messages/counter_number.rs",
    );
    for s in structs {
        println!("Found `Copy` struct: {}", s);
    }
}
