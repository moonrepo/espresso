use jpm_common::EsTarget;
use jpm_compiler::Compiler;
use jpm_package::Package;
use starbase_sandbox::create_sandbox;
use std::fs;

mod compile_modules {
    use super::*;

    #[tokio::test]
    async fn compiles_js_files_to_each_target() {
        let sandbox = create_sandbox("js-files");
        let package = Package::new(sandbox.path()).unwrap();
        let compiler = Compiler::new(&package).unwrap();

        for target in [EsTarget::Es2015, EsTarget::Es2018, EsTarget::Es2022] {
            let out_dir = compiler.compile(target).await.unwrap();

            assert_eq!(
                out_dir,
                sandbox.path().join(".jpm").join(target.to_string())
            );

            assert!(out_dir.join("index.mjs").exists());
            assert!(out_dir.join("helpers.mjs").exists());
        }

        assert_ne!(
            fs::read_to_string(sandbox.path().join(".jpm/es2015/helpers.mjs")).unwrap(),
            fs::read_to_string(sandbox.path().join(".jpm/es2022/helpers.mjs")).unwrap()
        );
    }
}
