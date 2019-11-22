use super::*;

impl IdentVisitor for syn::BareFnArg {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { name, ty, .. } = self;
        name.map(swap_tuple).visit().chain(ty.visit())
    }
}
