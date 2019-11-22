use super::*;

impl IdentVisitor for syn::ItemUnion {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, fields, .. } = self;
        ident.visit().chain(fields.visit())
    }
}
