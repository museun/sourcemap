use super::*;

impl IdentVisitor for syn::Item {
    fn visit(self) -> Vec<syn::Ident> {
        use syn::Item::*;
        match self {
            Const(item) => item.visit(),
            Enum(item) => item.visit(),
            ExternCrate(item) => item.visit(),
            Fn(item) => item.visit(),
            Impl(item) => item.visit(),
            Macro(item) => item.visit(),
            Macro2(item) => item.visit(),
            Mod(item) => item.visit(),
            Static(item) => item.visit(),
            Struct(item) => item.visit(),
            Trait(item) => item.visit(),
            TraitAlias(item) => item.visit(),
            Type(item) => item.visit(),
            Union(item) => item.visit(),
            _ => Default::default(),
        }
    }
}
