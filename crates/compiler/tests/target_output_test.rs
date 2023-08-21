mod utils;

use espresso_common::EsTarget;
use espresso_compiler::Compiler;
use espresso_package::Package;
use starbase_sandbox::{assert_snapshot, create_sandbox};
use utils::read_file;

macro_rules! test_target {
    ($method:ident, $target:expr) => {
        #[tokio::test]
        async fn $method() {
            let sandbox = create_sandbox("syntax");
            let package = Package::new(sandbox.path()).unwrap();
            let compiler = Compiler::new(&package).unwrap();
            let out_dir = compiler.compile($target).await.unwrap();

            assert_snapshot!(read_file(out_dir.join("index.mjs")));
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
        let compiler = Compiler::new(&package).unwrap();
        let out_dir = compiler.compile(EsTarget::Es2018).await.unwrap();

        assert_snapshot!(read_file(out_dir.join("index.mjs")));
        assert_snapshot!(read_file(out_dir.join("other.mjs")));
    }
}
