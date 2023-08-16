use crate::workspace_error::WorkspaceError;
use jpm_common::PackageName;
use jpm_package::Package;
use petgraph::algo::toposort;
use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use starbase_styles::color;
use std::collections::BTreeMap;
use tracing::{debug, trace};

// This is a simple DAG that represents the dependency graph of local
// packages within the workspace. Its primary use is for running processes
// in order (builds, etc), and is not used for dependency install.
pub struct PackageGraph<'ws> {
    graph: StableDiGraph<&'ws PackageName, ()>,
    indices: BTreeMap<&'ws PackageName, NodeIndex>,
    packages: &'ws BTreeMap<PackageName, Package>,
}

impl<'ws> PackageGraph<'ws> {
    pub fn new(packages: &BTreeMap<PackageName, Package>) -> PackageGraph {
        debug!("Creating a package graph with {} packages", packages.len());

        let mut graph = PackageGraph {
            graph: StableDiGraph::new(),
            indices: BTreeMap::new(),
            packages,
        };
        graph.add_packages();
        graph
    }

    pub fn toposort(&self) -> miette::Result<Vec<&PackageName>> {
        debug!("Sorting package graph topologically");

        match toposort(&self.graph, None) {
            Ok(indices) => {
                let names = indices
                    .into_iter()
                    .rev() // From most depended on to least
                    .map(|i| *self.graph.node_weight(i).unwrap())
                    .collect::<Vec<_>>();

                debug!(
                    "Sorted to: {}",
                    names
                        .iter()
                        .map(|n| color::id(n.as_str()))
                        .collect::<Vec<_>>()
                        .join(", ")
                );

                Ok(names)
            }
            Err(cycle) => Err(WorkspaceError::PackageGraphCycle {
                dep: (*self.graph.node_weight(cycle.node_id()).unwrap()).to_owned(),
            })?,
        }
    }

    fn add_packages(&mut self) {
        for (name, package) in self.packages {
            self.add_package(name, package);
        }

        self.indices.clear();
    }

    fn add_package(&mut self, name: &'ws PackageName, package: &'ws Package) -> NodeIndex {
        // Already inserted, skip
        if let Some(index) = self.indices.get(name) {
            return *index;
        }

        // Insert into the graph
        trace!(package = name.as_str(), "Adding package to graph");

        let index = self.graph.add_node(name);

        self.indices.insert(name, index);

        // Loop through dependencies and find packages in the current workspace
        let mut edges = vec![];

        let mut dependencies = BTreeMap::new();
        dependencies.extend(&package.manifest.dependencies);
        dependencies.extend(&package.manifest.dev_dependencies);

        for dep_name in dependencies.keys() {
            if let Some(dep_package) = self.packages.get(dep_name) {
                edges.push(self.add_package(dep_name, dep_package));

                trace!(
                    dependency = dep_name.as_str(),
                    package = name.as_str(),
                    "Linking dependency to package"
                );
            }
        }

        // Connect edges to the original index
        for edge in edges {
            self.graph.add_edge(index, edge, ());
        }

        index
    }
}
