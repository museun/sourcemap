use super::*;

impl IdentVisitor for syn::Signature {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident,
            inputs,
            output,
            ..
        } = self;
        ident.visit().chain(inputs.visit()).chain(output.visit())
    }
}

impl IdentVisitor for syn::FnArg {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::FnArg::Receiver(_) => Default::default(),
            syn::FnArg::Typed(ty) => ty.visit(),
        }
    }
}

impl IdentVisitor for syn::ReturnType {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::ReturnType::Type(_, ty) => ty.visit(),
            _ => Default::default(),
        }
    }
}
