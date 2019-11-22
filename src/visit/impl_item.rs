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
