use espresso_common::PackageName;
use espresso_workspace::{SelectQuery, Workspace};
use starbase_sandbox::{create_empty_sandbox, create_sandbox};

mod workspace {
    use super::*;

    #[test]
    #[should_panic(expected = "Unable to detect a package workspace root")]
    fn errors_no_root_found() {
        let sandbox = create_empty_sandbox();

        Workspace::load_from(sandbox.path()).unwrap();
    }

    #[test]
    fn finds_root_via_lockfile() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("jpm.lock", "{}");
        sandbox.create_file("jpm.toml", "[package]\nname = \"ns/root\"");
        sandbox.create_file("some/nested/jpm.toml", "[package]\nname = \"ns/branch\"");

        let workspace = Workspace::load_from(&sandbox.path().join("some/nested/path")).unwrap();

        assert_eq!(workspace.root, sandbox.path());
    }

    #[test]
    fn finds_root_via_manifest() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("jpm.toml", "[package]\nname = \"ns/test\"");

        let workspace = Workspace::load_from(&sandbox.path().join("some/nested/path")).unwrap();

        assert_eq!(workspace.root, sandbox.path());
    }

    mod polyrepo {
        use super::*;

        #[test]
        fn marks_as_polyrepo() {
            let sandbox = create_sandbox("polyrepo");
            let workspace = Workspace::load_from(sandbox.path()).unwrap();

            assert!(!workspace.monorepo);
        }

        #[test]
        fn loads_a_single_package() {
            let sandbox = create_sandbox("polyrepo");
            let workspace = Workspace::load_from(sandbox.path()).unwrap();
            let packages = workspace.load_packages().unwrap();
            let root = packages
                .get(&PackageName::parse("poly/root").unwrap())
                .unwrap();

            assert_eq!(root.root, sandbox.path());
        }

        #[test]
        fn always_selects_one_package_regardless_of_filters() {
            let sandbox = create_sandbox("polyrepo");
            let workspace = Workspace::load_from(sandbox.path()).unwrap();

            let packages = workspace.select_packages(SelectQuery::default()).unwrap();

            assert_eq!(packages.len(), 1);

            let packages = workspace
                .select_packages(SelectQuery {
                    all: true,
                    ..SelectQuery::default()
                })
                .unwrap();

            assert_eq!(packages.len(), 1);

            let packages = workspace
                .select_packages(SelectQuery {
                    names: Some(&vec![PackageName::parse("poly/root").unwrap()]),
                    ..SelectQuery::default()
                })
                .unwrap();

            assert_eq!(packages.len(), 1);

            let packages = workspace
                .select_packages(SelectQuery {
                    filters: Some(&vec!["poly/*".into()]),
                    ..SelectQuery::default()
                })
                .unwrap();

            assert_eq!(packages.len(), 1);
        }
    }

    mod monorepo {
        use super::*;

        #[test]
        fn marks_as_monorepo() {
            let sandbox = create_sandbox("monorepo");
            let workspace = Workspace::load_from(sandbox.path()).unwrap();

            assert!(workspace.monorepo);
        }

        #[test]
        fn loads_all_packages_matching_glob() {
            let sandbox = create_sandbox("monorepo");
            let workspace = Workspace::load_from(sandbox.path()).unwrap();
            let packages = workspace.load_packages().unwrap();

            // Only packages
            assert_eq!(packages.len(), 3);
            assert!(packages.contains_key(&PackageName::parse("mono/foo").unwrap()));
            assert!(packages.contains_key(&PackageName::parse("mono/bar").unwrap()));
            assert!(packages.contains_key(&PackageName::parse("mono/baz").unwrap()));
        }

        #[test]
        fn selects_all() {
            let sandbox = create_sandbox("monorepo");
            let workspace = Workspace::load_from(sandbox.path()).unwrap();

            let packages = workspace
                .select_packages(SelectQuery {
                    all: true,
                    ..SelectQuery::default()
                })
                .unwrap();

            assert_eq!(packages.len(), 3);
        }

        #[test]
        fn selects_by_name() {
            let sandbox = create_sandbox("monorepo");
            let workspace = Workspace::load_from(sandbox.path()).unwrap();

            let packages = workspace
                .select_packages(SelectQuery {
                    names: Some(&vec![
                        PackageName::parse("mono/foo").unwrap(),
                        PackageName::parse("mono/baz").unwrap(),
                    ]),
                    ..SelectQuery::default()
                })
                .unwrap();

            assert_eq!(packages.len(), 2);
            assert_eq!(packages[0].name(), "mono/baz");
            assert_eq!(packages[1].name(), "mono/foo");
        }

        #[test]
        fn selects_by_filter() {
            let sandbox = create_sandbox("monorepo");
            let workspace = Workspace::load_from(sandbox.path()).unwrap();

            let packages = workspace
                .select_packages(SelectQuery {
                    filters: Some(&vec!["*/ba{z,r}".into()]),
                    ..SelectQuery::default()
                })
                .unwrap();

            assert_eq!(packages.len(), 2);
            assert_eq!(packages[0].name(), "mono/bar");
            assert_eq!(packages[1].name(), "mono/baz");
        }

        #[test]
        fn selects_by_filter_with_negated() {
            let sandbox = create_sandbox("monorepo");
            let workspace = Workspace::load_from(sandbox.path()).unwrap();

            let packages = workspace
                .select_packages(SelectQuery {
                    filters: Some(&vec!["*/ba{z,r}".into(), "!*/bar".into()]),
                    ..SelectQuery::default()
                })
                .unwrap();

            assert_eq!(packages.len(), 1);
            assert_eq!(packages[0].name(), "mono/baz");
        }

        #[test]
        #[should_panic(expected = "No packages have been selected.")]
        fn errors_none_selected() {
            let sandbox = create_sandbox("monorepo");
            let workspace = Workspace::load_from(sandbox.path()).unwrap();

            workspace
                .select_packages(SelectQuery {
                    filters: Some(&vec!["*/unknown".into()]),
                    ..SelectQuery::default()
                })
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "The package mono/unknown doesn't exist")]
        fn errors_unknown_name() {
            let sandbox = create_sandbox("monorepo");
            let workspace = Workspace::load_from(sandbox.path()).unwrap();

            workspace
                .select_packages(SelectQuery {
                    names: Some(&vec![PackageName::parse("mono/unknown").unwrap()]),
                    ..SelectQuery::default()
                })
                .unwrap();
        }
    }
}
