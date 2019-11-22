use super::*;

impl IdentVisitor for syn::TypeImplTrait {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { bounds, .. } = self;
        bounds.visit()
    }
}
