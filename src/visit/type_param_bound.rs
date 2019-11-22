use super::*;

impl IdentVisitor for syn::TypeParamBound {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::TypeParamBound::Trait(trait_) => trait_.visit(),
            _ => Default::default(),
        }
    }
}
