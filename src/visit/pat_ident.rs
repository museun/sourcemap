use super::*;

impl IdentVisitor for syn::PatIdent {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, subpat, .. } = self;
        ident.visit().chain(subpat.visit())
    }
}
