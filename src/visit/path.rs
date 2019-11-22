use super::*;

impl IdentVisitor for syn::Path {
    fn visit(self) -> Vec<syn::Ident> {
        self.segments
            .last()
            .cloned()
            .map(|s| s.ident)
            .into_iter()
            .collect()
    }
}
