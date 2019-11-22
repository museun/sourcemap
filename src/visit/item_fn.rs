use super::*;

impl IdentVisitor for syn::ItemFn {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { sig, block, .. } = self;
        sig.visit().chain(block.visit())
    }
}
