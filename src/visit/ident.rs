use super::*;

impl IdentVisitor for syn::Ident {
    fn visit(self) -> Vec<syn::Ident> {
        vec![self]
    }
}
