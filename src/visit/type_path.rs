use super::*;

impl IdentVisitor for syn::TypePath {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { qself, path, .. } = self;
        qself.visit().chain(path.visit())
    }
}
