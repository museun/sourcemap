use super::*;

impl IdentVisitor for syn::ImplItemType {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, ty, .. } = self;
        ident.visit().chain(ty.visit())
    }
}
