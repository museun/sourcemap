use super::*;

impl IdentVisitor for syn::TypeTraitObject {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { bounds, .. } = self;
        bounds.visit()
    }
}
