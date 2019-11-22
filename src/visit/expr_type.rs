use super::*;

impl IdentVisitor for syn::ExprType {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { expr, .. } = self;
        expr.visit()
    }
}
