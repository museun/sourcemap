use super::*;

impl IdentVisitor for syn::PatOr {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { cases, .. } = self;
        cases.visit()
    }
}
