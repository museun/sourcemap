use super::*;

impl IdentVisitor for syn::TraitItemConst {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident, ty, default, ..
        } = self;
        ident.visit().chain(ty.visit()).chain(default.visit())
    }
}
