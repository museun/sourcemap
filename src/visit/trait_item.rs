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
