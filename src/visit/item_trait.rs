use super::*;

impl IdentVisitor for syn::ItemTrait {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident,
            supertraits,
            items,
            ..
        } = self;
        ident
            .visit()
            .chain(supertraits.visit())
            .chain(items.visit())
    }
}
