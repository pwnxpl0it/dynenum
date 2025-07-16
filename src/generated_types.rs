
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Information {
    Email,
    PhoneNumber,
    Credential,
    Date,
}

impl std::fmt::Display for Information {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Information::Email => "email",
            Information::PhoneNumber => "phone_number",
            Information::Credential => "credential",
            Information::Date => "date",
        };
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for Information {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "email" => Ok(Information::Email),
            "phone_number" => Ok(Information::PhoneNumber),
            "credential" => Ok(Information::Credential),
            "date" => Ok(Information::Date),
            _ => Err("Unknown type"),
        }
    }
}

impl AsRef<str> for Information {
    fn as_ref(&self) -> &str {
        match self {
            Information::Email => "email",
            Information::PhoneNumber => "phone_number",
            Information::Credential => "credential",
            Information::Date => "date",
        }
    }
}

/// All types as string slices
pub const ALL_INFORMATION: &[&str] = &["email", "phone_number", "credential", "date"];
