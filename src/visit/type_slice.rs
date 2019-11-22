use super::*;

impl IdentVisitor for syn::TypeSlice {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, .. } = self;
        elem.visit()
    }
}
