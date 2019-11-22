use super::*;

impl IdentVisitor for syn::Block {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { stmts, .. } = self;
        stmts.visit()
    }
}
