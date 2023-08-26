mod utils;

use espresso_common::EsTarget;
use espresso_compiler::Declarations;
use espresso_manifest::{BuildOptimizePng, ManifestBuild};
use espresso_package::Package;
use espresso_store::Store;
use starbase_sandbox::{assert_snapshot, create_sandbox, locate_fixture};
use std::fs;
use std::sync::Arc;
use utils::*;

mod compile_modules {
    use super::*;

    #[tokio::test]
    async fn compiles_js_files_to_each_target() {
        let sandbox = create_sandbox("js-files");
        let package = Package::new(sandbox.path()).unwrap();
        let compiler = create_compiler(sandbox.path(), &package);

        for target in [EsTarget::Es2015, EsTarget::Es2018, EsTarget::Es2022] {
            let out_dir = compiler.compile(target).await.unwrap();

            assert_eq!(
                out_dir,
                sandbox.path().join(".espm").join(target.to_string())
            );

            assert!(out_dir.join("index.mjs").exists());
            assert!(out_dir.join("helpers.mjs").exists());
        }

        assert_ne!(
            fs::read_to_string(sandbox.path().join(".espm/es2015/helpers.mjs")).unwrap(),
            fs::read_to_string(sandbox.path().join(".espm/es2022/helpers.mjs")).unwrap()
        );
    }
}

mod compile_assets {
    use super::*;

    #[tokio::test]
    async fn copies_non_js_files() {
        let sandbox = create_sandbox("assets");
        let package = Package::new(sandbox.path()).unwrap();
        let compiler = create_compiler(sandbox.path(), &package);
        let out_dir = compiler.compile(EsTarget::Es2015).await.unwrap();

        assert!(out_dir.join("cat.png").exists());
        assert!(out_dir.join("moon.svg").exists());
    }

    #[tokio::test]
    async fn optimizes_png() {
        let sandbox = create_sandbox("assets");
        let package = Package::new(sandbox.path()).unwrap();
        let compiler = create_compiler(sandbox.path(), &package);
        let out_dir = compiler.compile(EsTarget::Es2015).await.unwrap();

        assert_ne!(
            fs::metadata(out_dir.join("cat.png")).unwrap().len(),
            fs::metadata(locate_fixture("assets").join("src/cat.png"))
                .unwrap()
                .len()
        );
    }

    #[tokio::test]
    async fn optimizes_png_with_diff_level() {
        let sandbox = create_sandbox("assets");
        let mut package = Package::new(sandbox.path()).unwrap();

        package.manifest.build.optimize_png = BuildOptimizePng::Level(1);

        let base_size = fs::metadata(
            create_compiler(sandbox.path(), &package)
                .compile(EsTarget::Es2015)
                .await
                .unwrap()
                .join("cat.png"),
        )
        .unwrap()
        .len();

        package.manifest.build.optimize_png = BuildOptimizePng::Level(6);

        let next_size = fs::metadata(
            create_compiler(sandbox.path(), &package)
                .compile(EsTarget::Es2020)
                .await
                .unwrap()
                .join("cat.png"),
        )
        .unwrap()
        .len();

        assert_ne!(base_size, next_size);
    }
}

mod compile_declarations {
    use super::*;

    #[tokio::test]
    async fn generates_dmts_for_ts_files() {
        let sandbox = create_sandbox("ts-files");
        let package = Package::new(sandbox.path()).unwrap();
        let compiler = create_compiler(sandbox.path(), &package);

        let out_dir = compiler.compile(EsTarget::Es2018).await.unwrap();

        assert!(out_dir.join("../tsconfig.es2018.json").exists());
        assert!(out_dir.join("index.d.mts").exists());
        assert!(out_dir.join("helpers.d.mts").exists());

        // Ensure d.ts doesn't exist!
        assert!(!out_dir.join("index.d.ts").exists());
        assert!(!out_dir.join("helpers.d.ts").exists());
    }

    #[tokio::test]
    async fn doesnt_generate_dmts_for_js_files() {
        let sandbox = create_sandbox("js-files");
        let package = Package::new(sandbox.path()).unwrap();
        let compiler = create_compiler(sandbox.path(), &package);

        let out_dir = compiler.compile(EsTarget::Es2018).await.unwrap();

        assert!(!out_dir.join("../tsconfig.es2018.json").exists());
        assert!(!out_dir.join("index.d.mts").exists());
        assert!(!out_dir.join("helpers.d.mts").exists());
    }

    #[tokio::test]
    async fn can_use_a_custom_tsconfig() {
        let sandbox = create_sandbox("ts-files");

        let decl = Declarations::new(
            sandbox.path().to_path_buf(),
            sandbox.path().join(".espm"),
            Arc::new(ManifestBuild::default()),
            Arc::new(Store::load_from(sandbox.path()).unwrap()),
        );

        sandbox.create_file(
            "tsconfig.espm.json",
            r#"{
    "compilerOptions": {
        "strict": false,
    }
}"#,
        );

        let state = decl.create_tsconfig(&EsTarget::Es2022).unwrap();

        assert_snapshot!(read_file(state.path));
        assert!(!state.project_references);
    }

    #[tokio::test]
    async fn remaps_custom_tsconfig_paths() {
        let sandbox = create_sandbox("ts-files");

        let decl = Declarations::new(
            sandbox.path().to_path_buf(),
            sandbox.path().join(".espm"),
            Arc::new(ManifestBuild::default()),
            Arc::new(Store::load_from(sandbox.path()).unwrap()),
        );

        sandbox.create_file(
            "tsconfig.espm.json",
            r#"{
    "exclude": ["src/*.test.*", "tests/**/*"],
    "extends": ["../../tsconfig.options.json", "npm-package/tsconfig.json"],
    "compilerOptions": {
        "baseUrl": "src",
        "rootDirs": [".", "../types"],
        "paths": {
            "~": ["src/index.ts", "./src/*"]
        }
    },
    "references": [
        {
            "path": "../../common"
        }
    ]
}"#,
        );

        let state = decl.create_tsconfig(&EsTarget::Es2022).unwrap();

        assert_snapshot!(read_file(state.path));
        assert!(state.project_references);
    }
}
