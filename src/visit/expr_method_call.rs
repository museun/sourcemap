use super::*;

impl IdentVisitor for syn::ExprMethodCall {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            method,
            receiver,
            args,
            ..
        } = self;
        method.visit().chain(receiver.visit()).chain(args.visit())
    }
}
