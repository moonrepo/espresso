use swc_common::{errors::HANDLER, MultiSpan};
use swc_core::ecma::{
    ast::Ident,
    visit::{VisitMut, VisitMutWith},
};

pub struct DetectCjsVisitor;

impl DetectCjsVisitor {
    fn report_error<S: Into<MultiSpan>>(&self, span: S, value: &str) {
        HANDLER.with(|handler| {
            handler
                .struct_span_err(
                    span,
                    &format!("CommonJS is not supported, found `{}`", value),
                )
                .emit();
        });
    }
}

impl VisitMut for DetectCjsVisitor {
    fn visit_mut_ident(&mut self, n: &mut Ident) {
        n.visit_mut_children_with(self);

        if n.sym.eq_str_ignore_ascii_case("__dirname") {
            self.report_error(n.span, "__dirname");
        } else if n.sym.eq_str_ignore_ascii_case("__filename") {
            self.report_error(n.span, "__filename");
        }
    }
}
