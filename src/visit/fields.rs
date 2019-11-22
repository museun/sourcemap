use super::*;

impl IdentVisitor for syn::Fields {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::Fields::Named(fields) => fields.visit(),
            syn::Fields::Unnamed(fields) => fields.visit(),
            syn::Fields::Unit => Default::default(),
        }
    }
}
