use super::*;

impl IdentVisitor for syn::TypeGroup {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, .. } = self;
        elem.visit()
    }
}
