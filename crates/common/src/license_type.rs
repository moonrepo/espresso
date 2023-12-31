use schematic::{SchemaType, Schematic};
use serde::{Deserialize, Serialize};
use spdx::Expression;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct LicenseType(Expression);

impl Display for LicenseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

impl LicenseType {
    pub fn parse(value: &str) -> Result<Self, spdx::ParseError> {
        Ok(Self(Expression::parse(value)?))
    }
}

impl TryFrom<String> for LicenseType {
    type Error = spdx::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        LicenseType::parse(&value)
    }
}

impl Into<String> for LicenseType {
    fn into(self) -> String {
        self.0.to_string()
    }
}

impl Deref for LicenseType {
    type Target = Expression;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Eq for LicenseType {}

impl Schematic for LicenseType {
    fn generate_schema() -> SchemaType {
        SchemaType::string()
    }
}
