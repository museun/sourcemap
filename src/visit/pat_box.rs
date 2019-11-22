use super::*;

impl IdentVisitor for syn::PatBox {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { pat, .. } = self;
        pat.visit()
    }
}
