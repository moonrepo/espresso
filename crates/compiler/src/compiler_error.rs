use miette::Diagnostic;
use starbase_styles::{Style, Stylize};
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum CompilerError {}
