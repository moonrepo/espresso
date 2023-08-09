use jpm_common::EsTarget;
use jpm_compiler::{Compiler, CompilerError};
use jpm_package::Package;
use starbase_sandbox::create_empty_sandbox;

macro_rules! test_cjs {
    ($content:literal) => {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("src/index.js", $content);
        sandbox.create_file("jpm.toml", "[package]\nname = \"ns/detect-cjs\"");

        let package = Package::new(sandbox.path()).unwrap();
        let compiler = Compiler::new(&package).unwrap();

        if let Err(error) = compiler.compile(EsTarget::Es2015).await {
            match error.downcast::<CompilerError>().unwrap() {
                CompilerError::ModuleTransformFailed { error, .. } => {
                    panic!("{}", error);
                }
                _ => {}
            }
        }
    };
}

mod detect_cjs {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "CommonJS is not supported, found `__dirname`.")]
    async fn errors_on_dirname() {
        test_cjs!("console.log(__dirname);");
    }

    #[tokio::test]
    #[should_panic(expected = "CommonJS is not supported, found `__filename`.")]
    async fn errors_on_filename() {
        test_cjs!("console.log(__filename);");
    }

    #[tokio::test]
    #[should_panic(expected = "CommonJS is not supported, found `require()`.")]
    async fn errors_on_require() {
        test_cjs!("require('module');");
    }

    #[tokio::test]
    #[should_panic(expected = "CommonJS is not supported, found `require.resolve()`.")]
    async fn errors_on_require_resolve() {
        test_cjs!("require.resolve");
    }

    #[tokio::test]
    #[should_panic(expected = "CommonJS is not supported, found `require.resolve()`.")]
    async fn errors_on_require_resolve_call() {
        test_cjs!("require.resolve('module');");
    }

    #[tokio::test]
    #[should_panic(expected = "CommonJS is not supported, found `require.cache`.")]
    async fn errors_on_require_cache() {
        test_cjs!("require.cache;");
        test_cjs!("delete require.cache['module'];");
    }

    #[tokio::test]
    #[should_panic(expected = "CommonJS is not supported, found `require.extensions`.")]
    async fn errors_on_require_exts() {
        test_cjs!("require.extensions");
        test_cjs!("require.extensions['.js'] = {};");
    }
}
