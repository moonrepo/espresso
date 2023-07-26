use swc_core::ecma::{
    ast::{ExportAll, ImportDecl, NamedExport, Str},
    visit::{VisitMut, VisitMutWith},
};

pub struct AddMjsExtensionVisitor;

impl AddMjsExtensionVisitor {
    fn add_ext(&mut self, source: &mut Box<Str>) {
        // ./, ../, etc
        if source.value.starts_with('.') && !source.value.ends_with(".mjs") {
            source.value = format!("{}.mjs", source.value).into();
        }
    }
}

impl VisitMut for AddMjsExtensionVisitor {
    // export * from './file'
    fn visit_mut_export_all(&mut self, n: &mut ExportAll) {
        n.visit_mut_children_with(self);

        self.add_ext(&mut n.src);
    }

    // import './file'
    // import ... from './file'
    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        n.visit_mut_children_with(self);

        self.add_ext(&mut n.src);
    }

    // export {} from './file'
    // export * as ns from './file'
    fn visit_mut_named_export(&mut self, n: &mut NamedExport) {
        n.visit_mut_children_with(self);

        if let Some(src) = &mut n.src {
            self.add_ext(src);
        }
    }
}
