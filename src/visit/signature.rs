use super::*;

impl IdentVisitor for syn::Signature {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident,
            inputs,
            output,
            ..
        } = self;
        ident.visit().chain(inputs.visit()).chain(output.visit())
    }
}
