use super::*;

impl IdentVisitor for syn::Expr {
    fn visit(self) -> Vec<syn::Ident> {
        use syn::Expr::*;
        match self {
            Assign(assign) => assign.visit(),
            Block(block) => block.visit(),
            Call(call) => call.visit(),
            MethodCall(call) => call.visit(),
            Type(ty) => ty.visit(),
            Field(field) => field.visit(),
            _ => Default::default(),
        }
    }
}
