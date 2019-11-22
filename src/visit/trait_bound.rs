use super::*;

impl IdentVisitor for syn::TraitBound {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { path, .. } = self;
        path.visit()
    }
}
