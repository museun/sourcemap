use super::*;

impl IdentVisitor for syn::FieldsUnnamed {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { unnamed, .. } = self;
        unnamed.visit()
    }
}
