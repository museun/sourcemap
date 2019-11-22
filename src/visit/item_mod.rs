use super::*;

impl IdentVisitor for syn::ItemMod {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, content, .. } = self;
        ident.visit().chain(content.visit())
    }
}
