use super::*;

impl IdentVisitor for syn::ItemEnum {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident, variants, ..
        } = self;
        ident.visit().chain(variants.visit())
    }
}
