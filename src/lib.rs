//! dynenum: Dynamic enums from YAML for classification types
//!
//! This crate provides two modes:
//!
//! ## 1. Dynamic mode (default)
//! - Types are loaded from a YAML file at runtime.
//! - Use [`Type`] (a string wrapper) and [`load_types_from_yaml`].
//! - Example:
//!   ```no_run
//!   use dynenum::{Type, load_types_from_yaml, match_type};
//!   let types = load_types_from_yaml(std::path::Path::new("types.yaml")).unwrap();
//!   for t in &types {
//!       match_type!(t,
//!           "email" => { println!("Email! (dynamic)"); },
//!           _ => { println!("Other: {} (dynamic)", t) }
//!       );
//!   }
//!   ```
//!
//! ## 2. Static mode (enable with `features = ["static-types"]`)
//! - Types are generated as a real Rust enum at build time from `types.yaml`.
//! - The enum name is taken from the YAML key (e.g., `Information`).
//! - Use [`Information`] (an enum), [`ALL_INFORMATION`], and `FromStr`.
//! - Example:
//!   ```no_run
//!   use dynenum::{Information, ALL_INFORMATION};
//!   use std::str::FromStr;
//!   for t in ALL_INFORMATION {
//!       let info = Information::from_str(t).unwrap();
//!       match_type!(info,
//!           "email" => { println!("Email! (static)"); },
//!           _ => { println!("Other: {} (static)", info) }
//!       );
//!   }
//!   ```
//!
//! See the crate README for more details.

#[cfg(not(feature = "static-types"))]
use std::collections::HashSet;
#[cfg(not(feature = "static-types"))]
use std::fmt;
#[cfg(not(feature = "static-types"))]
use std::hash::{Hash, Hasher};
#[cfg(not(feature = "static-types"))]
use std::path::Path;
#[cfg(not(feature = "static-types"))]
use std::fs::File;
#[cfg(not(feature = "static-types"))]
use std::io::BufReader;

#[cfg(not(feature = "static-types"))]
mod dynamic_types {
    use super::*;
    use serde::Deserialize;
    /// A type loaded from YAML, behaving like a dynamic enum.
    ///
    /// Only available when the `static-types` feature is **not** enabled.
    #[derive(Clone, Eq)]
    pub struct Type(pub String);

    impl PartialEq for Type {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Hash for Type {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state)
        }
    }

    impl fmt::Debug for Type {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Type({:?})", self.0)
        }
    }

    impl fmt::Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl AsRef<str> for Type {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl From<&str> for Type {
        fn from(s: &str) -> Self {
            Type(s.to_string())
        }
    }

    impl From<String> for Type {
        fn from(s: String) -> Self {
            Type(s)
        }
    }

    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            Ok(Type(s))
        }
    }

    #[derive(Debug, Deserialize)]
    struct TypesYaml(pub std::collections::BTreeMap<String, Vec<String>>);
    /// Loads types from a YAML file.
    ///
    /// Only available when the `static-types` feature is **not** enabled.
    ///
    /// # Arguments
    /// * `path` - Path to the YAML file.
    ///
    /// # Returns
    /// * `HashSet<Type>` on success, or an error.
    ///
    /// # Example
    /// ```no_run
    /// use dynenum::load_types_from_yaml;
    /// let types = load_types_from_yaml(std::path::Path::new("types.yaml")).unwrap();
    /// ```
    pub fn load_types_from_yaml(path: &Path) -> Result<HashSet<Type>, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let parsed: TypesYaml = serde_yaml::from_reader(reader)?;
        let variants = parsed.0.values().next().ok_or("No enum key found in YAML")?;
        Ok(variants.iter().cloned().map(Type).collect())
    }
}

#[cfg(not(feature = "static-types"))]
pub use dynamic_types::{Type, load_types_from_yaml};

/// Macro for matching on Type as if it were an enum.
///
/// # Example (static mode)
/// ```rust
/// use dynenum::{Information, match_type};
/// use std::str::FromStr;
/// let info = Information::from_str("email").unwrap();
/// match_type!(info,
///     "email" => { println!("Email!"); },
///     _ => { println!("Other"); }
/// );
/// ```
#[macro_export]
macro_rules! match_type {
    // With default arm
    ($val:expr, $( $pat:literal => $body:tt ),+ , _ => $default:tt $(,)? ) => {
        match $val.as_ref() {
            $( $pat => $body, )+
            _ => $default,
        }
    };
    // Without default arm (optional)
    ($val:expr, $( $pat:literal => $body:tt ),+ $(,)? ) => {
        match $val.as_ref() {
            $( $pat => $body, )+
        }
    };
}

#[cfg(feature = "static-types")]
mod static_types {
    //!
    //! # Static mode
    //!
    //! When the `static-types` feature is enabled, `Type` is a generated enum.
    //!
    //! - Use [`ALL_TYPES`] for all possible types.
    //! - Use `FromStr` to parse from string.
    //! - Use `Display`/`AsRef<str>` to get the string value.
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_types.rs"));
}

#[cfg(feature = "static-types")]
pub use static_types::*;

#[cfg(not(feature = "static-types"))]
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_type_traits() {
        let a = Type("email".to_string());
        let b = Type("email".to_string());
        let c = Type("phone".to_string());
        assert_eq!(a, b);
        assert_ne!(a, c);
        let mut set = HashSet::new();
        set.insert(a.clone());
        assert!(set.contains(&b));
        assert_eq!(format!("{}", a), "email");
        assert_eq!(format!("{:?}", a), "Type(\"email\")");
        let a2 = a.clone();
        assert_eq!(a, a2);
    }

    #[test]
    fn test_type_deserialize() {
        let yaml = "---\nemail\n";
        let ty: Type = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(ty, Type("email".to_string()));
    }

    #[test]
    fn test_load_types_from_yaml() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("types.yaml");
        let yaml = "types:\n  - email\n  - phone\n  - date\n";
        let mut file = File::create(&file_path).unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
        let set = load_types_from_yaml(&file_path).unwrap();
        let expected: HashSet<_> = [
            Type("email".to_string()),
            Type("phone".to_string()),
            Type("date".to_string()),
        ].into_iter().collect();
        assert_eq!(set, expected);
    }

    #[test]
    fn test_match_type_macro() {
        let t = Type("email".to_string());
        let mut called = false;
        match_type!(t,
            "email" => { called = true; },
            "phone" => { let _ = called; panic!("Should not match"); },
            _ => { let _ = called; panic!("Should not reach default"); }
        );
        assert!(called);

        let t = Type("unknown".to_string());
        let mut default_called = false;
        match_type!(t,
            "email" => { /* do nothing */ },
            "phone" => { /* do nothing */ },
            _ => { default_called = true; }
        );
        assert!(default_called);
    }
}
