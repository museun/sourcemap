use super::*;

impl IdentVisitor for syn::Pat {
    fn visit(self) -> Vec<syn::Ident> {
        use syn::Pat::*;
        match self {
            Box(pat) => pat.visit(),
            Ident(ident) => ident.visit(),
            Lit(lit) => lit.visit(),
            Or(or) => or.visit(),
            Range(range) => range.visit(),
            Reference(ref_) => ref_.visit(),
            Slice(slice) => slice.visit(),
            Struct(fields) => fields.visit(),
            Tuple(tuple) => tuple.visit(),
            TupleStruct(tuple) => tuple.visit(),
            Type(ty) => ty.visit(),
            _ => Default::default(),
        }
    }
}

impl IdentVisitor for syn::PatBox {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { pat, .. } = self;
        pat.visit()
    }
}

impl IdentVisitor for syn::PatType {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { pat, .. } = self;
        pat.visit()
    }
}

impl IdentVisitor for syn::PatIdent {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, subpat, .. } = self;
        ident.visit().chain(subpat.visit())
    }
}

impl IdentVisitor for syn::PatLit {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { expr, .. } = self;
        expr.visit()
    }
}

impl IdentVisitor for syn::PatOr {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { cases, .. } = self;
        cases.visit()
    }
}

impl IdentVisitor for syn::PatRange {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { lo, hi, .. } = self;
        lo.visit().chain(hi.visit())
    }
}

impl IdentVisitor for syn::PatReference {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { pat, .. } = self;
        pat.visit()
    }
}

impl IdentVisitor for syn::PatSlice {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elems, .. } = self;
        elems.visit()
    }
}

impl IdentVisitor for syn::PatStruct {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { fields, .. } = self;
        fields.visit()
    }
}

impl IdentVisitor for syn::PatTupleStruct {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { pat, .. } = self;
        pat.visit()
    }
}

impl IdentVisitor for syn::PatTuple {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elems, .. } = self;
        elems.visit()
    }
}
