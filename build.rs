use std::env;
use std::fs;
use std::path::Path;
use convert_case::Casing;

fn main() {
    // Only generate static types if the feature is enabled
    if std::env::var_os("CARGO_FEATURE_STATIC_TYPES").is_some() {
        let out_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/generated_types.rs");
        let yaml_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("types.yaml");
        let yaml = fs::read_to_string(&yaml_path).expect("Failed to read types.yaml at crate root");
        let doc: serde_yaml::Value = serde_yaml::from_str(&yaml).expect("Invalid YAML in types.yaml");
        let (enum_name, variants) = doc.as_mapping()
            .and_then(|map| map.iter().next())
            .and_then(|(k, v)| {
                let enum_name = k.as_str()?;
                let variants = v.as_sequence()?;
                Some((enum_name, variants))
            })
            .expect("YAML must have a single key (enum name) mapping to a sequence of variants");
        let enum_ident = enum_name.to_case(convert_case::Case::Pascal);
        let const_ident = format!("ALL_{}", enum_name.to_case(convert_case::Case::UpperSnake));
        let mut enum_variants = String::new();
        let mut display_arms = String::new();
        let mut fromstr_arms = String::new();
        let mut asref_arms = String::new();
        let mut all_variants = Vec::new();
        for v in variants {
            let s = v.as_str().expect("Variant must be a string");
            let variant = s.to_case(convert_case::Case::Pascal);
            enum_variants.push_str(&format!("    {},\n", variant));
            display_arms.push_str(&format!("            {enum_ident}::{variant} => \"{s}\",\n"));
            fromstr_arms.push_str(&format!("            \"{s}\" => Ok({enum_ident}::{variant}),\n"));
            asref_arms.push_str(&format!("            {enum_ident}::{variant} => \"{s}\",\n"));
            all_variants.push(format!("\"{}\"", s));
        }
        let all_types = all_variants.join(", ");
        let generated = format!(r#"
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum {enum_ident} {{
{enum_variants}}}

impl std::fmt::Display for {enum_ident} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        let s = match self {{
{display_arms}        }};
        write!(f, "{{}}", s)
    }}
}}

impl std::str::FromStr for {enum_ident} {{
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        match s {{
{fromstr_arms}            _ => Err("Unknown type"),
        }}
    }}
}}

impl AsRef<str> for {enum_ident} {{
    fn as_ref(&self) -> &str {{
        match self {{
{asref_arms}        }}
    }}
}}

/// All types as string slices
pub const {const_ident}: &[&str] = &[{all_types}];
"#,
            enum_ident = enum_ident,
            enum_variants = enum_variants,
            display_arms = display_arms,
            fromstr_arms = fromstr_arms,
            asref_arms = asref_arms,
            const_ident = const_ident,
            all_types = all_types,
        );
        fs::write(&out_path, generated).expect("Failed to write generated_types.rs");
        println!("cargo:rerun-if-changed={}", yaml_path.display());
    }
} 