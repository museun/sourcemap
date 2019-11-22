use super::*;

impl IdentVisitor for syn::TypePtr {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, .. } = self;
        elem.visit()
    }
}
