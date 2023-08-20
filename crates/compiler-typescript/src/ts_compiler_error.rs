use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum TsCompilerError {
    #[diagnostic(code(compiler::javascript::no_runtime))]
    #[error("Failed to detect a JavaScript runtime. Unable to generate TypeScript declarations!")]
    NoRuntime,
}
