use super::*;

impl IdentVisitor for syn::ExprAssign {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { left, right, .. } = self;
        left.visit().chain(right.visit())
    }
}
