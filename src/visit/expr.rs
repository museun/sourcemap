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

impl IdentVisitor for syn::ExprAssign {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { left, right, .. } = self;
        left.visit().chain(right.visit())
    }
}

impl IdentVisitor for syn::ExprType {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { expr, .. } = self;
        expr.visit()
    }
}

impl IdentVisitor for syn::ExprBlock {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { block, .. } = self;
        block.stmts.visit()
    }
}

impl IdentVisitor for syn::ExprCall {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { func, args, .. } = self;
        func.visit().chain(args.visit())
    }
}

impl IdentVisitor for syn::ExprField {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { base, .. } = self;
        base.visit()
    }
}

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
