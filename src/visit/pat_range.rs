use super::*;

impl IdentVisitor for syn::PatRange {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { lo, hi, .. } = self;
        lo.visit().chain(hi.visit())
    }
}
