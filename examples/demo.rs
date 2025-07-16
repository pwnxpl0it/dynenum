//! Demo for dynenum crate

#[cfg(not(feature = "static-types"))]
use dynenum::load_types_from_yaml;

#[cfg(feature = "static-types")]
use dynenum::{Type, ALL_TYPES};
#[cfg(feature = "static-types")]
use std::str::FromStr;

#[cfg(not(feature = "static-types"))]
fn main() {
    use std::path::Path;
    println!("Dynamic mode (String-backed):");
    let types = load_types_from_yaml(Path::new("types.yaml")).expect("Failed to load types.yaml");
    for t in &types {
        dynenum::match_type!(t,
            "email" => { println!("Email! (dynamic)"); },
            "phone" => { println!("Phone! (dynamic)"); },
            _ => { println!("Other: {} (dynamic)", t) }
        );
    }
}
