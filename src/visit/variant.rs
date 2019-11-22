use super::*;

impl IdentVisitor for syn::Variant {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident,
            fields,
            discriminant,
            ..
        } = self;
        ident
            .visit()
            .chain(fields.visit())
            .chain(discriminant.visit())
    }
}
