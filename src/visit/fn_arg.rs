use super::*;

impl IdentVisitor for syn::FnArg {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::FnArg::Receiver(_) => Default::default(),
            syn::FnArg::Typed(ty) => ty.visit(),
        }
    }
}
