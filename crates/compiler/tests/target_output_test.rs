mod utils;

use espresso_common::EsTarget;
use espresso_package::Package;
use starbase_sandbox::{assert_snapshot, create_sandbox};
use utils::*;

macro_rules! test_target {
    ($method:ident, $target:expr) => {
        mod $method {
            use super::*;

            #[tokio::test]
            async fn transforms_modules() {
                let sandbox = create_sandbox("syntax");
                let package = Package::new(sandbox.path()).unwrap();
                let compiler = create_compiler(sandbox.path(), &package);

                let out_dir = compiler.compile($target).await.unwrap();

                assert_snapshot!(read_file(out_dir.join("index.mjs")));
            }

            #[tokio::test]
            async fn creates_tsconfig() {
                let sandbox = create_sandbox("syntax");
                let package = Package::new(sandbox.path()).unwrap();
                let compiler = create_compiler(sandbox.path(), &package);

                let out_dir = compiler.compile($target).await.unwrap();
                let tsconfig = format!("tsconfig.{}.json", $target.to_string());

                assert_snapshot!(read_file(out_dir.join("..").join(tsconfig)));
            }
        }
    };
}

mod target_output {
    use super::*;

    test_target!(es2015, EsTarget::Es2015);
    test_target!(es2016, EsTarget::Es2016);
    test_target!(es2017, EsTarget::Es2017);
    test_target!(es2018, EsTarget::Es2018);
    test_target!(es2019, EsTarget::Es2019);
    test_target!(es2020, EsTarget::Es2020);
    test_target!(es2021, EsTarget::Es2021);
    test_target!(es2022, EsTarget::Es2022);

    #[tokio::test]
    async fn supports_legacy_decorators() {
        let sandbox = create_sandbox("syntax-legacy-decorators");
        let package = Package::new(sandbox.path()).unwrap();
        let compiler = create_compiler(sandbox.path(), &package);
        let out_dir = compiler.compile(EsTarget::Es2018).await.unwrap();

        assert_snapshot!(read_file(out_dir.join("index.mjs")));
        assert_snapshot!(read_file(out_dir.join("other.mjs")));
        assert_snapshot!(read_file(out_dir.join("../tsconfig.es2018.json")));
    }
}
