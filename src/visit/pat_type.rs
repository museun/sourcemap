use super::*;

impl IdentVisitor for syn::PatType {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { pat, .. } = self;
        pat.visit()
    }
}
