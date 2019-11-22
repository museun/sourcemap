use super::*;

impl IdentVisitor for syn::ImplItemMacro {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { mac, .. } = self;
        mac.visit()
    }
}
