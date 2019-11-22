use super::*;

impl IdentVisitor for syn::ItemMacro {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, mac, .. } = self;
        ident.visit().chain(mac.visit())
    }
}
