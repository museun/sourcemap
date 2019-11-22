use super::*;

impl IdentVisitor for syn::PatTupleStruct {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { pat, .. } = self;
        pat.visit()
    }
}
