use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};

use anyhow::Result;
use lazy_static::lazy_static;
use syn::*;
use tempfile::NamedTempFile;

lazy_static! {
    static ref TYPE_MAP: HashMap<String, String> = HashMap::from_iter(
        [
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
            ("u128", "long long"),
            ("usize", "size_t"),
            ("f32", "float"),
            ("f64", "double"),
            ("char", "char"),
            ("bool", "bool"),
        ]
        .into_iter()
        .map(|(x, y)| (x.to_string(), y.to_string()))
    );
}

pub fn convert(src: &str, dest: Option<&str>, format: bool) -> Result<String> {
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
            Fields::Unnamed(f) => handle_unnamed(&f, &variant_name),
            Fields::Named(f) => handle_named(&f, &variant_name),
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
        "#include <stdint.h>
        #include <stdbool.h>
        typedef uint8_t empty;

        {tag}

        struct {enum_name} {{
            enum {enum_name}Tag variant;
            union {{ {}
            }};
        }};
        ",
        c_code
    );

    // Write to file
    let code = if format { format_code(&code)? } else { code };
    if let Some(path) = dest {
        let mut outfile = fs::File::create(path)?;
        outfile.write_all(code.as_bytes())?;
    } else {
        println!("{code}");
    }

    Ok(code)
}

fn get_type(ty: &Type) -> String {
    let ty = match ty {
        Type::Path(TypePath { path, .. }) => path
            .clone()
            .segments
            .into_iter()
            .map(|seg| seg.ident.to_string())
            .collect::<String>(),
        _ => unimplemented!("ty: {:?}", ty),
    };
    match TYPE_MAP.get(&*ty) {
        Some(t) => t.to_string(),
        None => panic!("Rust type '{}' cannot be mapped to a C type", ty),
    }
}

fn handle_unnamed(fields: &FieldsUnnamed, variant_name: &str) -> String {
    let c_fields = fields
        .unnamed
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, f)| {
            let ty = get_type(&f.ty);
            format!("{} t_{i};", ty)
        })
        .collect::<Vec<String>>()
        .join("\n");
    format!(
        "
        struct {{
            {c_fields}
        }} {};",
        variant_name.to_lowercase()
    )
}

fn handle_named(fields: &FieldsNamed, variant_name: &str) -> String {
    let c_fields = fields
        .named
        .clone()
        .into_iter()
        .map(|f| {
            let ty = get_type(&f.ty);
            let ident = f.ident.unwrap().to_string();
            format!("{} {};", ty, ident)
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "
        struct {{
            {c_fields}
        }} {};",
        variant_name.to_lowercase()
    )
}

fn format_code(s: &str) -> Result<String> {
    let mut inf = NamedTempFile::new()?;
    let mut outf = NamedTempFile::new()?;
    inf.write_all(s.as_bytes())?;

    std::process::Command::new("indent")
        .args([
            inf.path().display().to_string(),
            "-o".into(),
            outf.path().display().to_string(),
        ])
        .output()?;

    let mut formatted = String::new();
    outf.read_to_string(&mut formatted)?;
    Ok(formatted)
}

#[cfg(test)]
mod test {
    use super::*;
    use paste::paste;

    macro_rules! test (
        ($n:literal) => {
            paste! {
                #[test]
                fn [< test_ $n >] () {
                    let c = convert(&format!("examples/{:02}.rs", $n), None).unwrap();
                    println!("{}", format_code(&c).unwrap());
                }
            }
        }
    );

    test!(1);
    test!(2);
    test!(3);
    test!(4);
    test!(5);
    test!(6);
}
