use super::*;

impl IdentVisitor for syn::TraitItemMacro {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { mac, .. } = self;
        mac.visit()
    }
}
