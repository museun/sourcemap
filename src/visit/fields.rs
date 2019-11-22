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

impl IdentVisitor for syn::FieldsNamed {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { named, .. } = self;
        named.visit()
    }
}

impl IdentVisitor for syn::FieldsUnnamed {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { unnamed, .. } = self;
        unnamed.visit()
    }
}

impl IdentVisitor for syn::Field {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, ty, .. } = self;
        ident.visit().chain(ty.visit())
    }
}

impl IdentVisitor for syn::Member {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::Member::Named(ident) => ident.visit(),
            _ => Default::default(),
        }
    }
}
impl IdentVisitor for syn::FieldPat {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { member, pat, .. } = self;
        member.visit().chain(pat.visit())
    }
}
