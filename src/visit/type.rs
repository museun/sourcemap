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

impl IdentVisitor for syn::TypeArray {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, len, .. } = self;
        elem.visit().chain(len.visit())
    }
}

impl IdentVisitor for syn::TypeBareFn {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { inputs, output, .. } = self;
        inputs.visit().chain(output.visit())
    }
}

impl IdentVisitor for syn::BareFnArg {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { name, ty, .. } = self;
        name.map(swap_tuple).visit().chain(ty.visit())
    }
}

impl IdentVisitor for syn::TypeGroup {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, .. } = self;
        elem.visit()
    }
}

impl IdentVisitor for syn::TypeImplTrait {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { bounds, .. } = self;
        bounds.visit()
    }
}

impl IdentVisitor for syn::TypeMacro {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { mac, .. } = self;
        mac.visit()
    }
}

impl IdentVisitor for syn::TypeParamBound {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::TypeParamBound::Trait(trait_) => trait_.visit(),
            _ => Default::default(),
        }
    }
}

impl IdentVisitor for syn::TypeParen {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, .. } = self;
        elem.visit()
    }
}

impl IdentVisitor for syn::TypePath {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { qself, path, .. } = self;
        qself.visit().chain(path.visit())
    }
}

impl IdentVisitor for syn::TypePtr {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, .. } = self;
        elem.visit()
    }
}

impl IdentVisitor for syn::TypeReference {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, .. } = self;
        elem.visit()
    }
}
impl IdentVisitor for syn::TypeSlice {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, .. } = self;
        elem.visit()
    }
}
impl IdentVisitor for syn::TypeTraitObject {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { bounds, .. } = self;
        bounds.visit()
    }
}

impl IdentVisitor for syn::TypeTuple {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elems, .. } = self;
        elems.visit()
    }
}
