use super::*;

impl IdentVisitor for syn::ReturnType {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::ReturnType::Type(_, ty) => ty.visit(),
            _ => Default::default(),
        }
    }
}
