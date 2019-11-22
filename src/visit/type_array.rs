use super::*;

impl IdentVisitor for syn::TypeArray {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, len, .. } = self;
        elem.visit().chain(len.visit())
    }
}
