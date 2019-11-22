use super::*;

impl IdentVisitor for syn::FieldPat {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { member, pat, .. } = self;
        member.visit().chain(pat.visit())
    }
}
