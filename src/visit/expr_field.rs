use super::*;

impl IdentVisitor for syn::ExprField {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { base, .. } = self;
        base.visit()
    }
}
