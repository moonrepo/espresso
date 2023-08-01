use std::path::PathBuf;
use swc_core::ecma::{
    ast::{CallExpr, ExportAll, Expr, ImportDecl, Lit, NamedExport, Str},
    visit::{VisitMut, VisitMutWith},
};

pub struct AddMjsExtensionVisitor;

fn add_ext(source: &mut Str) {
    // .
    if source.value.eq(".") {
        source.value = "./index.mjs".into();
        source.raw = None;

        // ./, ../
    } else if source.value.starts_with('.')
        // Only append to those without extensions
        && PathBuf::from(source.value.to_string())
            .extension()
            .is_none()
    {
        source.value = format!("{}.mjs", source.value).into();
        source.raw = None;
    }
}

impl VisitMut for AddMjsExtensionVisitor {
    // export * from './file'
    fn visit_mut_export_all(&mut self, n: &mut ExportAll) {
        n.visit_mut_children_with(self);
        add_ext(&mut n.src);
    }

    // import './file'
    // import ... from './file'
    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        n.visit_mut_children_with(self);
        add_ext(&mut n.src);
    }

    // export {} from './file'
    // export * as ns from './file'
    fn visit_mut_named_export(&mut self, n: &mut NamedExport) {
        n.visit_mut_children_with(self);

        if let Some(src) = &mut n.src {
            add_ext(src);
        }
    }

    // import("./file");
    fn visit_mut_call_expr(&mut self, n: &mut CallExpr) {
        n.visit_mut_children_with(self);

        if n.callee.is_import() && n.args.len() == 1 {
            if let Expr::Lit(Lit::Str(arg)) = &mut (*n.args[0].expr) {
                add_ext(arg);
            }
        }
    }
}
