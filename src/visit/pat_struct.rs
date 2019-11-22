use super::*;

impl IdentVisitor for syn::PatStruct {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { fields, .. } = self;
        fields.visit()
    }
}
