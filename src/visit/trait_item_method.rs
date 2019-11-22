use super::*;

impl IdentVisitor for syn::TraitItemMethod {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { sig, default, .. } = self;
        sig.visit().chain(default.visit())
    }
}
