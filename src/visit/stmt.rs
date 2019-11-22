use super::*;

impl IdentVisitor for syn::Stmt {
    fn visit(self) -> Vec<syn::Ident> {
        use syn::Stmt::*;
        match self {
            Expr(expr) | Semi(expr, _) => expr.visit(),
            Local(local) => local.visit(),
            Item(item) => item.visit(),
        }
    }
}
