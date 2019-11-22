use super::*;

impl IdentVisitor for syn::ItemConst {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, expr, .. } = self;
        ident.visit().chain(expr.visit())
    }
}
