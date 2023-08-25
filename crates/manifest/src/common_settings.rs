use espresso_common::{PackageName, VersionReq};
use std::collections::BTreeMap;

pub type ManifestDependencies = BTreeMap<PackageName, VersionReq>;
