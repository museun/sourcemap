use super::*;

impl IdentVisitor for syn::TraitItemType {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident,
            bounds,
            default,
            ..
        } = self;
        ident.visit().chain(bounds.visit()).chain(default.visit())
    }
}
