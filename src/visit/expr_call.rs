use super::*;

impl IdentVisitor for syn::ExprCall {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { func, args, .. } = self;
        func.visit().chain(args.visit())
    }
}
