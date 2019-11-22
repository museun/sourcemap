use super::*;

impl IdentVisitor for syn::FieldsNamed {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { named, .. } = self;
        named.visit()
    }
}
