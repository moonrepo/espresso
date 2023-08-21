use espresso_workspace::{PackageGraph, Workspace};
use starbase_sandbox::create_sandbox;

mod package_graph {
    use super::*;

    #[test]
    #[should_panic(expected = "detected a dependency cycle")]
    fn errors_for_cycle() {
        let sandbox = create_sandbox("graph-cycle");
        let workspace = Workspace::load_from(sandbox.path()).unwrap();

        PackageGraph::new(workspace.load_packages().unwrap())
            .toposort()
            .unwrap();
    }

    #[test]
    fn sorts_topologically() {
        let sandbox = create_sandbox("graph");
        let workspace = Workspace::load_from(sandbox.path()).unwrap();

        let graph = PackageGraph::new(workspace.load_packages().unwrap());
        let names = graph
            .toposort()
            .unwrap()
            .iter()
            .map(|n| n.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            names,
            vec!["graph/dd", "graph/aa", "graph/ee", "graph/ff", "graph/cc", "graph/bb"]
        );
    }
}
