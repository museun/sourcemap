use super::*;

impl IdentVisitor for syn::Member {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::Member::Named(ident) => ident.visit(),
            _ => Default::default(),
        }
    }
}
