#![allow(clippy::from_over_into)]

mod license_type;
mod package_name;
mod target;

pub const OUT_DIR: &str = ".espm";

pub use license_type::*;
pub use package_name::*;
pub use semver::{Version, VersionReq};
pub use target::*;
