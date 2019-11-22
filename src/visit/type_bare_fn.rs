use super::*;

impl IdentVisitor for syn::TypeBareFn {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { inputs, output, .. } = self;
        inputs.visit().chain(output.visit())
    }
}
