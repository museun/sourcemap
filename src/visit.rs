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
mod impl_item_const;
mod impl_item_macro;
mod impl_item_method;
mod impl_item_type;
mod item;
mod item_const;
mod item_enum;
mod item_extern_crate;
mod item_fn;
mod item_impl;
mod item_macro;
mod item_macro2;
mod item_mod;
mod item_static;
mod item_struct;
mod item_trait;
mod item_trait_alias;
mod item_type;
mod item_union;
mod local;
mod r#macro;
mod member;
mod pat;
mod pat_box;
mod pat_ident;
mod pat_lit;
mod pat_or;
mod pat_range;
mod pat_reference;
mod pat_slice;
mod pat_struct;
mod pat_tuple;
mod pat_tuple_struct;
mod pat_type;
mod path;
mod q_self;
mod return_type;
mod signature;
mod stmt;
mod trait_bound;
mod trait_item;
mod trait_item_const;
mod trait_item_macro;
mod trait_item_method;
mod trait_item_type;
mod r#type;
mod type_array;
mod type_bare_fn;
mod type_group;
mod type_impl_trait;
mod type_macro;
mod type_param_bound;
mod type_paren;
mod type_path;
mod type_ptr;
mod type_reference;
mod type_slice;
mod type_trait_object;
mod type_tuple;
mod variant;

pub use {
    bare_fn_arg::*, block::*, expr::*, expr_assign::*, expr_block::*, expr_call::*, expr_field::*,
    expr_method_call::*, expr_type::*, field::*, field_pat::*, fields::*, fields_named::*,
    fields_unnamed::*, fn_arg::*, ident::*, impl_item::*, impl_item_const::*, impl_item_macro::*,
    impl_item_method::*, impl_item_type::*, item::*, item_const::*, item_enum::*,
    item_extern_crate::*, item_fn::*, item_impl::*, item_macro::*, item_macro2::*, item_mod::*,
    item_static::*, item_struct::*, item_trait::*, item_trait_alias::*, item_type::*,
    item_union::*, local::*, member::*, pat::*, pat_box::*, pat_ident::*, pat_lit::*, pat_or::*,
    pat_range::*, pat_reference::*, pat_slice::*, pat_struct::*, pat_tuple::*, pat_tuple_struct::*,
    pat_type::*, path::*, q_self::*, r#macro::*, r#type::*, return_type::*, signature::*, stmt::*,
    trait_bound::*, trait_item::*, trait_item_const::*, trait_item_macro::*, trait_item_method::*,
    trait_item_type::*, type_array::*, type_bare_fn::*, type_group::*, type_impl_trait::*,
    type_macro::*, type_param_bound::*, type_paren::*, type_path::*, type_ptr::*,
    type_reference::*, type_slice::*, type_trait_object::*, type_tuple::*, variant::*,
};
