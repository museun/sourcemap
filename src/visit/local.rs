use super::*;

impl IdentVisitor for syn::Local {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { pat, init, .. } = self;
        pat.visit().chain(init.visit())
    }
}
