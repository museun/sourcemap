use super::*;

impl IdentVisitor for syn::ImplItemMethod {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { sig, block, .. } = self;
        sig.visit().chain(block.visit())
    }
}
