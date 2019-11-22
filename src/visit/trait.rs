use super::*;

impl IdentVisitor for syn::TraitItem {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::TraitItem::Const(item) => item.visit(),
            syn::TraitItem::Method(item) => item.visit(),
            syn::TraitItem::Type(item) => item.visit(),
            syn::TraitItem::Macro(item) => item.visit(),
            _ => Default::default(),
        }
    }
}

impl IdentVisitor for syn::TraitBound {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { path, .. } = self;
        path.visit()
    }
}

impl IdentVisitor for syn::TraitItemConst {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident, ty, default, ..
        } = self;
        ident.visit().chain(ty.visit()).chain(default.visit())
    }
}

impl IdentVisitor for syn::TraitItemMacro {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { mac, .. } = self;
        mac.visit()
    }
}

impl IdentVisitor for syn::TraitItemMethod {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { sig, default, .. } = self;
        sig.visit().chain(default.visit())
    }
}

impl IdentVisitor for syn::TraitItemType {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident,
            bounds,
            default,
            ..
        } = self;
        ident.visit().chain(bounds.visit()).chain(default.visit())
    }
}
