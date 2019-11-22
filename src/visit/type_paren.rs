use super::*;

impl IdentVisitor for syn::TypeParen {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, .. } = self;
        elem.visit()
    }
}
