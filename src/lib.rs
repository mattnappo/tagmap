use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;
use syn::*;

pub mod examples;

pub fn convert(src: &str, dest: &str) -> Result<()> {
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
    let c_variants = enum_.variants.into_iter().map(|v| {
        let variant_name = v.ident.to_string();
        let c_code = match v.fields {
            Fields::Unnamed(f) => handle_unnamed(&f),
            Fields::Named(f) => handle_named(&f),
            Fields::Unit => handle_unit(),
        };

        // Make this return string of (variant type variant name)
        (variant_name, c_code)
    });

    // Now make the tag
    let variants = c_variants
        .map(|(name, _)| format!("{}_{}", enum_name.to_uppercase(), name.to_uppercase()))
        .collect::<Vec<String>>()
        .join(",");

    let tag = format!("enum {}Tag {{ {} }};", enum_name, variants);

    // Write to file
    let mut outfile = fs::File::open(dest)?;

    let code = format!(
        "
        #include <stdint.h>
        typedef empty uint8_t;

        {tag}

        struct {enum_name} {{
            enum {enum_name}Tag variant;
            union {{
                {variant_type} {variant_name};
            }};
        }}
        ",
    );

    outfile.write_all(code.as_bytes())?;
}

fn handle_unnamed(fields: &FieldsUnnamed) -> Vec<String> {
    todo!()
}
fn handle_named(fields: &FieldsNamed) -> Vec<String> {
    todo!()
}
fn handle_unit() -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_() {}
}
