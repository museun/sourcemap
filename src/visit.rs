pub trait Combine {
    type Item;
    fn chain<C>(self, other: C) -> Self
    where
        C: IntoIterator<Item = Self::Item>;
}

impl<T> Combine for Vec<T> {
    type Item = T;
    fn chain<C>(mut self, other: C) -> Self
    where
        C: IntoIterator<Item = Self::Item>,
    {
        self.extend(other.into_iter());
        self
    }
}

pub trait IdentVisitor {
    fn visit(self) -> Vec<syn::Ident>;
}

impl<T> IdentVisitor for Box<T>
where
    T: IdentVisitor,
{
    fn visit(self) -> Vec<syn::Ident> {
        (*self).visit()
    }
}

impl<E, T> IdentVisitor for (E, T)
where
    T: IdentVisitor,
{
    fn visit(self) -> Vec<syn::Ident> {
        let (_, this) = self;
        this.visit()
    }
}

impl<T> IdentVisitor for Option<T>
where
    T: IdentVisitor,
{
    fn visit(self) -> Vec<syn::Ident> {
        self.map(IdentVisitor::visit)
            .into_iter()
            .flatten()
            .collect()
    }
}

impl<T> IdentVisitor for Vec<T>
where
    T: IdentVisitor,
{
    fn visit(self) -> Vec<syn::Ident> {
        self.into_iter().flat_map(IdentVisitor::visit).collect()
    }
}

impl<E, T> IdentVisitor for syn::punctuated::Punctuated<E, T>
where
    E: IdentVisitor,
{
    fn visit(self) -> Vec<syn::Ident> {
        self.into_iter().flat_map(IdentVisitor::visit).collect()
    }
}

pub(self) fn swap_tuple<T, E>((left, right): (T, E)) -> (E, T) {
    (right, left)
}

mod bare_fn_arg;
mod block;
mod expr;
mod expr_assign;
mod expr_block;
mod expr_call;
mod expr_field;
mod expr_method_call;
mod expr_type;
mod field;
mod field_pat;
mod fields;
mod fields_named;
mod fields_unnamed;
mod fn_arg;
mod ident;
mod impl_item;
mod item;
mod r#macro;
mod member;
mod pat;
mod path;
mod q_self;
mod return_type;
mod signature;
mod stmt;
mod r#trait;
mod r#type;
mod variant;

pub use {
    bare_fn_arg::*, block::*, expr::*, expr_assign::*, expr_block::*, expr_call::*, expr_field::*,
    expr_method_call::*, expr_type::*, field::*, field_pat::*, fields::*, fields_named::*,
    fields_unnamed::*, fn_arg::*, ident::*, impl_item::*, item::*, member::*, pat::*, path::*,
    q_self::*, r#macro::*, r#trait::*, r#type::*, return_type::*, signature::*, stmt::*,
    variant::*,
};
