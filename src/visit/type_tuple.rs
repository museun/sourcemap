use super::*;

impl IdentVisitor for syn::TypeTuple {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elems, .. } = self;
        elems.visit()
    }
}
