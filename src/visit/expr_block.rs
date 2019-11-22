use super::*;

impl IdentVisitor for syn::ExprBlock {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { block, .. } = self;
        block.stmts.visit()
    }
}
