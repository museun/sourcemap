use super::*;

impl IdentVisitor for syn::Ident {
    fn visit(self) -> Vec<syn::Ident> {
        vec![self]
    }
}

impl IdentVisitor for syn::Path {
    fn visit(self) -> Vec<syn::Ident> {
        self.segments
            .last()
            .cloned()
            .map(|s| s.ident)
            .into_iter()
            .collect()
    }
}

impl IdentVisitor for syn::Block {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { stmts, .. } = self;
        stmts.visit()
    }
}

impl IdentVisitor for syn::Macro {
    fn visit(self) -> Vec<syn::Ident> {
        self.path.visit()
    }
}

impl IdentVisitor for syn::QSelf {
    fn visit(self) -> Vec<syn::Ident> {
        self.ty.visit()
    }
}
