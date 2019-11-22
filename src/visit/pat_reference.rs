use super::*;

impl IdentVisitor for syn::PatReference {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { pat, .. } = self;
        pat.visit()
    }
}
