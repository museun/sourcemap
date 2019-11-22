use super::*;

impl IdentVisitor for syn::PatTuple {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elems, .. } = self;
        elems.visit()
    }
}
