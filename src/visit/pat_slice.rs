use super::*;

impl IdentVisitor for syn::PatSlice {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elems, .. } = self;
        elems.visit()
    }
}
