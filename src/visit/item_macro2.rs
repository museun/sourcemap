use super::*;

impl IdentVisitor for syn::ItemMacro2 {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, .. } = self;
        ident.visit()
    }
}
