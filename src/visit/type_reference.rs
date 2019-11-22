use super::*;

impl IdentVisitor for syn::TypeReference {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, .. } = self;
        elem.visit()
    }
}
