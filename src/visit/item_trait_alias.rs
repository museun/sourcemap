use super::*;

impl IdentVisitor for syn::ItemTraitAlias {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, bounds, .. } = self;
        ident.visit().chain(bounds.visit())
    }
}
