use super::*;

impl IdentVisitor for syn::ItemStatic {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident, ty, expr, ..
        } = self;
        ident.visit().chain(ty.visit()).chain(expr.visit())
    }
}
