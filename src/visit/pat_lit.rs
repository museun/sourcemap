use super::*;

impl IdentVisitor for syn::PatLit {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { expr, .. } = self;
        expr.visit()
    }
}
