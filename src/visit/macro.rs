use super::*;

impl IdentVisitor for syn::Macro {
    fn visit(self) -> Vec<syn::Ident> {
        self.path.visit()
    }
}
