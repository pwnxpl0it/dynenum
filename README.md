# 🦄 dynenum

**Dynamic & Static Rust enums from YAML!**

---

## ✨ What is this?

`dynenum` lets you define enum-like types in a YAML file and use them in Rust—either as a dynamic string wrapper or as a real Rust enum generated at build time!

- 📝 **Write your types in YAML**
- 🦀 **Use them as enums in Rust**
- 🔄 **Choose dynamic (runtime) or static (build-time) mode**

---

## 📦 YAML Example

```yaml
Information:
  - email
  - phone_number
  - credential
  - date
```

---

## 🚀 Usage

### 1️⃣ Dynamic Mode (default)

- Types are loaded at runtime from YAML.
- Use the `Type` struct and `load_types_from_yaml`.

```rust
use dynenum::{Type, load_types_from_yaml, match_type};
use std::path::Path;

let types = load_types_from_yaml(Path::new("types.yaml")).unwrap();
for t in &types {
    match_type!(t,
        "email" => { println!("Email! (dynamic)"); },
        _ => { println!("Other: {} (dynamic)", t) }
    );
}
```

### 2️⃣ Static Mode (build with `--features static-types`)

- The enum name is taken from the YAML key (e.g., `Information`).
- Use the generated enum and `ALL_<ENUMNAME>` constant.

```rust
use dynenum::{Information, ALL_INFORMATION};
use std::str::FromStr;

for t in ALL_INFORMATION {
    let info = Information::from_str(t).unwrap();
    match_type!(info,
        "email" => { println!("Email! (static)"); },
        _ => { println!("Other: {} (static)", info) }
    );
}
```

Build with:
```sh
cargo run --example static_usage --features static-types
```

---

## 🛠️ Features

- 🔄 **Dynamic mode:** String-backed, loads from YAML at runtime.
- 🏗️ **Static mode:** Real Rust enum generated at build time from YAML.
- 🧩 **Easy matching:** Use the `match_type!` macro for ergonomic matching.
- 🧑‍💻 **YAML-driven:** Change your types by editing YAML, not Rust code!

---

## 💡 Tips

- The enum name and constant are taken from the YAML key (e.g., `Information` → `Information`, `ALL_INFORMATION`).
- The macro `match_type!` works in both modes.
- The YAML file must be at the crate root and named `types.yaml`.
- In static mode, the enum and constant are auto-generated at build time—no need to check them in!

---

## 🦾 Example YAML → Rust

```yaml
Fruit:
  - apple
  - banana
  - orange
```

Generates:
```rust
pub enum Fruit {
    Apple,
    Banana,
    Orange,
}
pub const ALL_FRUIT: &[&str] = &["apple", "banana", "orange"];
```

---

## 🧪 Examples

- See [`examples/demo.rs`](examples/demo.rs) for dynamic mode.
- See [`examples/static_usage.rs`](examples/static_usage.rs) for static mode.

---

## ❤️ Enjoy fast, flexible, YAML-powered enums in Rust! 