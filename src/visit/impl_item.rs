use super::*;

impl IdentVisitor for syn::ImplItem {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::ImplItem::Const(item) => item.visit(),
            syn::ImplItem::Method(item) => item.visit(),
            syn::ImplItem::Type(item) => item.visit(),
            syn::ImplItem::Macro(item) => item.visit(),
            _ => Default::default(),
        }
    }
}

impl IdentVisitor for syn::ImplItemType {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, ty, .. } = self;
        ident.visit().chain(ty.visit())
    }
}

impl IdentVisitor for syn::ImplItemConst {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident, ty, expr, ..
        } = self;
        ident.visit().chain(ty.visit()).chain(expr.visit())
    }
}

impl IdentVisitor for syn::ImplItemMacro {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { mac, .. } = self;
        mac.visit()
    }
}

impl IdentVisitor for syn::ImplItemMethod {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { sig, block, .. } = self;
        sig.visit().chain(block.visit())
    }
}
