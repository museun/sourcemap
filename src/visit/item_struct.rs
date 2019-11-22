use super::*;

impl IdentVisitor for syn::ItemStruct {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, fields, .. } = self;
        ident.visit().chain(fields.visit())
    }
}
