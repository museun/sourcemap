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

impl IdentVisitor for syn::Local {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { pat, init, .. } = self;
        pat.visit().chain(init.visit())
    }
}
