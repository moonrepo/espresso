use swc_core::common::{errors::HANDLER, MultiSpan};
use swc_core::ecma::{
    ast::{CallExpr, Callee, Expr, Ident, MemberExpr, MemberProp},
    visit::{VisitMut, VisitMutWith},
};

pub struct DetectCjsVisitor;

impl DetectCjsVisitor {
    fn report_error<S: Into<MultiSpan>>(&self, span: S, value: &str) {
        HANDLER.with(|handler| {
            handler
                .struct_span_err(
                    span,
                    &format!("CommonJS is not supported, found `{}`.", value),
                )
                .emit();
        });
    }
}

impl VisitMut for DetectCjsVisitor {
    // __dirname
    // __filename
    fn visit_mut_ident(&mut self, n: &mut Ident) {
        n.visit_mut_children_with(self);

        if n.sym.eq_str_ignore_ascii_case("__dirname") {
            self.report_error(n.span, "__dirname");
        } else if n.sym.eq_str_ignore_ascii_case("__filename") {
            self.report_error(n.span, "__filename");
        }
    }

    // require()
    fn visit_mut_call_expr(&mut self, e: &mut CallExpr) {
        e.visit_mut_children_with(self);

        if let Callee::Expr(callee) = &e.callee {
            if let Expr::Ident(ident) = &**callee {
                if ident.sym.eq_str_ignore_ascii_case("require") && e.args.len() == 1 {
                    self.report_error(ident.span, "require()");
                }
            }
        }
    }

    // require.resolve()
    // require.cache
    // require.extensions
    // require.main
    fn visit_mut_member_expr(&mut self, e: &mut MemberExpr) {
        e.visit_mut_children_with(self);

        if let Expr::Ident(ident) = &*e.obj {
            if ident.sym.eq_str_ignore_ascii_case("require") {
                if let MemberProp::Ident(ident) = &e.prop {
                    if ident.sym.eq_str_ignore_ascii_case("resolve") {
                        self.report_error(ident.span, "require.resolve()");
                    } else if ident.sym.eq_str_ignore_ascii_case("cache") {
                        self.report_error(ident.span, "require.cache");
                    } else if ident.sym.eq_str_ignore_ascii_case("extensions") {
                        self.report_error(ident.span, "require.extensions");
                    } else if ident.sym.eq_str_ignore_ascii_case("main") {
                        self.report_error(ident.span, "require.main");
                    }
                };
            }
        }
    }
}
