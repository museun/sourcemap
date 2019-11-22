use super::*;

impl IdentVisitor for syn::QSelf {
    fn visit(self) -> Vec<syn::Ident> {
        self.ty.visit()
    }
}
