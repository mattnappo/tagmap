use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::bail;
use anyhow::Result;
use syn::*;

pub mod examples;

const TYPE_MAP: &[(&str, &str)] = &[
    ("String", "char*"),
    ("i8", "int8_t"),
    ("i16", "int16_t"),
    ("i32", "int32_t"),
    ("i64", "int64_t"),
    ("i128", "i128"),
    ("isize", "isize_t"),
    ("u8", "uint8_t"),
    ("u16", "uint16_t"),
    ("u32", "uint32_t"),
    ("u64", "uint64_t"),
    ("u128", "u128"),
    ("usize", "size_t"),
    ("f32", "float"),
    ("f64", "double"),
    ("char", "char"),
    ("bool", "bool"),
];

pub fn convert(src: &str, dest: Option<&str>) -> Result<String> {
    // Parse the file
    let mut fd = fs::File::open(src)?;
    let mut file = String::new();
    fd.read_to_string(&mut file)?;
    let file = syn::parse_file(&file)?;

    // Get all the enums
    let enums = file
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::Enum(e) => Some(e),
            _ => None,
        })
        .collect::<Vec<ItemEnum>>();

    let enum_ = enums[0].clone();

    // First, convert each variant of the enum accordingly
    // TODO: eventually we will map over all enums
    let enum_name = enum_.ident.to_string();
    let variants = enum_.variants.into_iter().map(|v| {
        let variant_name = v.ident.to_string();
        // This is a vector of strings of the form
        // "{variant c type} {variant name};"
        // So these can just be shoved into a union
        let c_code: String = match v.fields {
            Fields::Unnamed(f) => handle_unnamed(&f),
            Fields::Named(f) => handle_named(&f),
            Fields::Unit => format!("empty {};", variant_name.to_lowercase()),
        };

        (variant_name, c_code)
    });

    // Now make the tag
    let tag_variants = variants
        .clone()
        .map(|(name, _)| format!("{}_{}", enum_name.to_uppercase(), name.to_uppercase()))
        .collect::<Vec<String>>()
        .join(", ");

    let c_code = variants
        .map(|(_, code)| code)
        .collect::<Vec<String>>()
        .join("\n");

    let tag = format!("enum {}Tag {{ {} }};", enum_name, tag_variants);

    // Final C code output
    let code = format!(
        "
        #include <stdint.h>
        #include <stdbool.h>
        typedef uint8_t empty;

        {tag}

        struct {enum_name} {{
            enum {enum_name}Tag variant;
            union {{
                {}
            }};
        }};
        ",
        c_code
    );

    // Write to file
    if let Some(path) = dest {
        let mut outfile = fs::File::create(path)?;
        outfile.write_all(code.as_bytes())?;
    }

    Ok(code)
}

fn handle_unnamed(fields: &FieldsUnnamed) -> String {
    todo!("unnamed variant")
}

fn handle_named(fields: &FieldsNamed) -> String {
    let map = HashMap::<&str, &str>::from_iter(TYPE_MAP.to_owned());
    let c_fields = fields
        .named
        .clone()
        .into_iter()
        .map(|f| {
            let ty = match f.ty {
                Type::Path(TypePath { path, .. }) => path
                    .segments
                    .into_iter()
                    .map(|seg| seg.ident.to_string())
                    .collect::<String>(),
                _ => unimplemented!("ty: {:?}", f.ty),
            };

            let mapped_ty = match map.get(&*ty) {
                Some(t) => t.to_string(),
                None => panic!("Rust type {} cannot be mapped to a C type", ty),
            };
            let ident = f.ident.unwrap().to_string();
            format!("{} {};", mapped_ty, ident)
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "
        struct {{
            {c_fields}
        }};"
    )
}

#[cfg(test)]
mod test {
    use super::*;

    /*
    macro_rules! test (
        ($n:literal) => {
            let c = convert(format!("examples/{n:02}.rs"), "").unwrap();
            println!("{c}");
        }
    );
    */

    #[test]
    fn test_ex01() {
        let c = convert("examples/01.rs", None).unwrap();
        println!("{c}");
    }

    #[test]
    fn test_ex05() {
        let c = convert("examples/05.rs", Some("/home/matt/.tmp/f.c")).unwrap();
        println!("{c}");
    }
}
