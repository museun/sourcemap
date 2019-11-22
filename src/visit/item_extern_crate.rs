use super::*;

impl IdentVisitor for syn::ItemExternCrate {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, rename, .. } = self;
        ident.visit().chain(rename.visit())
    }
}
