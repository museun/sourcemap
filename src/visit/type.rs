use super::*;

impl IdentVisitor for syn::Type {
    fn visit(self) -> Vec<syn::Ident> {
        use syn::Type::*;
        match self {
            Array(array) => array.visit(),
            BareFn(bare) => bare.visit(),
            Group(group) => group.visit(),
            ImplTrait(impl_) => impl_.visit(),
            Macro(mac) => mac.visit(),
            Paren(paren) => paren.visit(),
            Path(path) => path.visit(),
            Ptr(ptr) => ptr.visit(),
            Reference(ref_) => ref_.visit(),
            Slice(slice) => slice.visit(),
            TraitObject(trait_) => trait_.visit(),
            Tuple(tuple) => tuple.visit(),
            _ => Default::default(),
        }
    }
}
