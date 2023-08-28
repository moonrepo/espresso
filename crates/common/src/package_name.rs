use miette::Diagnostic;
use once_cell::sync::Lazy;
use regex::Regex;
use schematic::{validate::HasLength, SchemaType, Schematic};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum PackageNameError {
    #[diagnostic(code(package::name::not_empty))]
    #[error("Package name must not be empty.")]
    Empty,

    #[diagnostic(code(package::name::no_repeating_dashes))]
    #[error("Repeating dashes are not allowed in package names.")]
    NoRepeatingDashes,

    #[diagnostic(code(package::name::missing_namespace))]
    #[error("Missing namespace from package name.")]
    MissingNamespace,

    #[diagnostic(code(package::name::invalid_namespace))]
    #[error("Only alpha-numeric characters and dashes are allowed in the package namespace.")]
    InvalidNamespace,

    #[diagnostic(code(package::name::namespace_length))]
    #[error("Package namespace (left of /) must be 2-32 characters in length.")]
    NamespaceLength,

    #[diagnostic(code(package::name::invalid_name))]
    #[error("Only alpha-numeric characters and dashes are allowed in the package name.")]
    InvalidName,

    #[diagnostic(code(package::name::name_length))]
    #[error("Package name (right of /) must be 2-32 characters in length.")]
    NameLength,
}

pub static NAME_SEPARATOR: char = '/';

pub static COMPONENT_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new("^([a-z][a-z0-9-]{0,30}[a-z0-9])$").unwrap());

pub static REPEATING_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new("(-{2,})").unwrap());

fn components(value: &str) -> (&str, &str) {
    let mut comps = value.split(NAME_SEPARATOR);

    (comps.next().unwrap(), comps.next().unwrap())
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct PackageName(String);

impl PackageName {
    pub fn parse(value: &str) -> Result<Self, PackageNameError> {
        if value.is_empty() {
            return Err(PackageNameError::Empty);
        } else if !value.contains(NAME_SEPARATOR) {
            return Err(PackageNameError::MissingNamespace);
        } else if REPEATING_PATTERN.is_match(value) {
            return Err(PackageNameError::NoRepeatingDashes);
        }

        let (namespace, package) = components(value);

        // Check namespace first
        if namespace.len() < 2 || namespace.len() > 32 {
            return Err(PackageNameError::NamespaceLength);
        }

        if !COMPONENT_PATTERN.is_match(namespace) {
            return Err(PackageNameError::InvalidNamespace);
        }

        // Then check package
        if package.len() < 2 || package.len() > 32 {
            return Err(PackageNameError::NameLength);
        }

        if !COMPONENT_PATTERN.is_match(package) {
            return Err(PackageNameError::InvalidName);
        }

        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn components(&self) -> (&str, &str) {
        components(&self.0)
    }

    pub fn namespace(&self) -> &str {
        self.components().0
    }

    pub fn package(&self) -> &str {
        self.components().1
    }
}

impl Display for PackageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for PackageName {
    type Err = PackageNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PackageName::parse(s)
    }
}

impl TryFrom<String> for PackageName {
    type Error = PackageNameError;

    fn try_from(value: String) -> Result<Self, PackageNameError> {
        PackageName::parse(&value)
    }
}

impl Into<String> for PackageName {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<str> for PackageName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<String> for PackageName {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsRef<PackageName> for PackageName {
    fn as_ref(&self) -> &PackageName {
        self
    }
}

impl HasLength for PackageName {
    fn length(&self) -> usize {
        self.0.len()
    }
}

impl Schematic for PackageName {
    fn generate_schema() -> SchemaType {
        SchemaType::string()
    }
}
