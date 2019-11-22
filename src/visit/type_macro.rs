use super::*;

impl IdentVisitor for syn::TypeMacro {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { mac, .. } = self;
        mac.visit()
    }
}
