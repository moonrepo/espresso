use espresso_common::PackageName;

mod package_name {
    use super::*;

    #[test]
    fn supports_all_namespace_patterns() {
        PackageName::parse("ns/package").unwrap();
        PackageName::parse("n-s/package").unwrap();
        PackageName::parse("namespace/package").unwrap();
        PackageName::parse("ns123/package").unwrap();
        PackageName::parse("n-s-1/package").unwrap();
        PackageName::parse("name-space-123/package").unwrap();

        PackageName::parse("ns/pkg-name").unwrap();
        PackageName::parse("ns/p123g").unwrap();
        PackageName::parse("ns/p-k-g").unwrap();
        PackageName::parse("ns/p-1-k-2-g-3").unwrap();
    }

    #[test]
    #[should_panic(expected = "Empty")]
    fn error_empty() {
        PackageName::parse("").unwrap();
    }

    mod namespace {
        use super::*;

        #[test]
        #[should_panic(expected = "MissingNamespace")]
        fn error_missing_namespace() {
            PackageName::parse("package").unwrap();
        }

        #[test]
        #[should_panic(expected = "NamespaceLength")]
        fn error_namespace_empty() {
            PackageName::parse("/package").unwrap();
        }

        #[test]
        #[should_panic(expected = "NamespaceLength")]
        fn error_namespace_to_short() {
            PackageName::parse("n/package").unwrap();
        }

        #[test]
        #[should_panic(expected = "NamespaceLength")]
        fn error_namespace_to_long() {
            PackageName::parse("nameabcdefghijklmnopqrstuvwxyzspace/package").unwrap();
        }

        #[test]
        #[should_panic(expected = "InvalidNamespace")]
        fn error_namespace_underscores() {
            PackageName::parse("n_s/package").unwrap();
        }

        #[test]
        #[should_panic(expected = "InvalidNamespace")]
        fn error_namespace_periods() {
            PackageName::parse("n.s/package").unwrap();
        }

        #[test]
        #[should_panic(expected = "InvalidNamespace")]
        fn error_namespace_uppercase() {
            PackageName::parse("Ns/package").unwrap();
        }

        #[test]
        #[should_panic(expected = "InvalidNamespace")]
        fn error_namespace_start_with_number() {
            PackageName::parse("1ns/package").unwrap();
        }

        #[test]
        #[should_panic(expected = "InvalidNamespace")]
        fn error_namespace_start_with_dash() {
            PackageName::parse("-ns/package").unwrap();
        }

        #[test]
        #[should_panic(expected = "InvalidNamespace")]
        fn error_namespace_end_with_dash() {
            PackageName::parse("ns-/package").unwrap();
        }

        #[test]
        #[should_panic(expected = "NoRepeatingDashes")]
        fn error_namespace_repeating_dashes() {
            PackageName::parse("n--s/package").unwrap();
        }
    }

    mod name {
        use super::*;

        #[test]
        #[should_panic(expected = "NameLength")]
        fn error_name_empty() {
            PackageName::parse("ns/").unwrap();
        }

        #[test]
        #[should_panic(expected = "NameLength")]
        fn error_name_to_short() {
            PackageName::parse("ns/p").unwrap();
        }

        #[test]
        #[should_panic(expected = "NameLength")]
        fn error_name_to_long() {
            PackageName::parse("ns/pkgabcdefghijklmnopqrstuvwxyzname").unwrap();
        }

        #[test]
        #[should_panic(expected = "InvalidName")]
        fn error_name_underscores() {
            PackageName::parse("ns/pkg_").unwrap();
        }

        #[test]
        #[should_panic(expected = "InvalidName")]
        fn error_name_periods() {
            PackageName::parse("ns/p.kg").unwrap();
        }

        #[test]
        #[should_panic(expected = "InvalidName")]
        fn error_name_uppercase() {
            PackageName::parse("ns/pKg").unwrap();
        }

        #[test]
        #[should_panic(expected = "InvalidName")]
        fn error_name_start_with_number() {
            PackageName::parse("ns/1package").unwrap();
        }

        #[test]
        #[should_panic(expected = "InvalidName")]
        fn error_name_start_with_dash() {
            PackageName::parse("ns/-package").unwrap();
        }

        #[test]
        #[should_panic(expected = "InvalidName")]
        fn error_name_end_with_dash() {
            PackageName::parse("ns/package-").unwrap();
        }

        #[test]
        #[should_panic(expected = "NoRepeatingDashes")]
        fn error_name_repeating_dashes() {
            PackageName::parse("ns/pac--kage").unwrap();
        }
    }
}
