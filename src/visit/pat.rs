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
