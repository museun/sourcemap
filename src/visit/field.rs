use super::*;

impl IdentVisitor for syn::Field {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, ty, .. } = self;
        ident.visit().chain(ty.visit())
    }
}
