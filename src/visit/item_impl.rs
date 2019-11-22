use super::*;

impl IdentVisitor for syn::ItemImpl {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            trait_,
            self_ty,
            items,
            ..
        } = self;
        trait_
            .map(|(_, path, _)| path)
            .visit()
            .chain(self_ty.visit())
            .chain(items.visit())
    }
}
