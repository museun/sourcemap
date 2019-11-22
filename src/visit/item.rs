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

impl IdentVisitor for syn::ItemUnion {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, fields, .. } = self;
        ident.visit().chain(fields.visit())
    }
}

impl IdentVisitor for syn::ItemConst {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, expr, .. } = self;
        ident.visit().chain(expr.visit())
    }
}

impl IdentVisitor for syn::ItemEnum {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident, variants, ..
        } = self;
        ident.visit().chain(variants.visit())
    }
}

impl IdentVisitor for syn::Variant {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident,
            fields,
            discriminant,
            ..
        } = self;
        ident
            .visit()
            .chain(fields.visit())
            .chain(discriminant.visit())
    }
}

impl IdentVisitor for syn::ItemExternCrate {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, rename, .. } = self;
        ident.visit().chain(rename.visit())
    }
}

impl IdentVisitor for syn::ItemFn {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { sig, block, .. } = self;
        sig.visit().chain(block.visit())
    }
}

impl IdentVisitor for syn::ItemImpl {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            trait_,
            self_ty,
            items,
            ..
        } = self;
        trait_
            .map(|(_, path, _)| path)
            .visit()
            .chain(self_ty.visit())
            .chain(items.visit())
    }
}

impl IdentVisitor for syn::ItemMacro {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, mac, .. } = self;
        ident.visit().chain(mac.visit())
    }
}

impl IdentVisitor for syn::ItemMacro2 {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, .. } = self;
        ident.visit()
    }
}

impl IdentVisitor for syn::ItemMod {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, content, .. } = self;
        ident.visit().chain(content.visit())
    }
}

impl IdentVisitor for syn::ItemStatic {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident, ty, expr, ..
        } = self;
        ident.visit().chain(ty.visit()).chain(expr.visit())
    }
}

impl IdentVisitor for syn::ItemStruct {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, fields, .. } = self;
        ident.visit().chain(fields.visit())
    }
}

impl IdentVisitor for syn::ItemTraitAlias {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, bounds, .. } = self;
        ident.visit().chain(bounds.visit())
    }
}

impl IdentVisitor for syn::ItemTrait {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident,
            supertraits,
            items,
            ..
        } = self;
        ident
            .visit()
            .chain(supertraits.visit())
            .chain(items.visit())
    }
}

impl IdentVisitor for syn::ItemType {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, ty, .. } = self;
        ident.visit().chain(ty.visit())
    }
}
