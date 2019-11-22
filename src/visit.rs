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

mod expr;
mod fields;
mod ident;
mod impl_item;
mod item;
mod pat;
mod signature;
mod stmt;
mod r#trait;
mod r#type;

pub use {
    expr::*, fields::*, ident::*, impl_item::*, item::*, pat::*, r#trait::*, r#type::*,
    signature::*, stmt::*,
};
