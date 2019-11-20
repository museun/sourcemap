#![allow(dead_code)]
#![feature(proc_macro_span)]
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};

// TODO
type Result<T> = std::result::Result<T, String>;

#[derive(Debug)]
struct Blob<'a> {
    pub object: &'a str,
    pub path: &'a Path,
}

impl<'a> From<&Blob<'a>> for BlobOwned {
    fn from(blob: &Blob<'a>) -> Self {
        Self {
            object: blob.object.to_owned(),
            path: blob.path.to_owned(),
        }
    }
}

#[derive(Debug)]
struct BlobOwned {
    pub object: String,
    pub path: PathBuf,
}

fn git_tree(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let output = std::process::Command::new("git")
        .current_dir(path)
        .args(&["ls-tree", "-zr", "HEAD"])
        .output()
        .map_err(|err| err.to_string())?;

    if !output.status.success() {
        return Err("git ls-tree failed".into());
    }

    Ok(output.stdout)
}

fn parse_blobs(data: &[u8]) -> impl Iterator<Item = Blob> {
    data.split(|&b| b == 0)
        .flat_map(|data| std::str::from_utf8(data))
        .filter_map(|line| line.split(' ').nth(2).map(|l| l.split('\t')))
        .filter_map(|mut parts| {
            Blob {
                object: parts.next()?,
                path: Path::new(parts.next()?),
            }
            .into()
        })
        .filter(|blob| blob.path.extension().and_then(|s| s.to_str()) == Some("rs"))
}

fn scry(source: impl AsRef<str>) -> Option<Vec<syn::Ident>> {
    syn::parse_str::<syn::File>(source.as_ref())
        .ok()?
        .items
        .visit()
        .into()
}

trait Combine {
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

trait IdentVisitor {
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

impl IdentVisitor for syn::Stmt {
    fn visit(self) -> Vec<syn::Ident> {
        use syn::Stmt::*;
        match self {
            Expr(expr) | Semi(expr, _) => expr.visit(),
            Local(local) => local.visit(),
            Item(item) => item.visit(),
        }
    }
}

impl IdentVisitor for syn::Type {
    fn visit(self) -> Vec<syn::Ident> {
        use syn::Type::*;
        match self {
            Array(array) => array.visit(),
            BareFn(bare) => bare.visit(),
            Group(group) => group.visit(),
            ImplTrait(impl_) => impl_.visit(),
            Macro(mac) => mac.visit(),
            Paren(paren) => paren.visit(),
            Path(path) => path.visit(),
            Ptr(ptr) => ptr.visit(),
            Reference(ref_) => ref_.visit(),
            Slice(slice) => slice.visit(),
            TraitObject(trait_) => trait_.visit(),
            Tuple(tuple) => tuple.visit(),
            _ => Default::default(),
        }
    }
}

impl IdentVisitor for syn::Item {
    fn visit(self) -> Vec<syn::Ident> {
        use syn::Item::*;
        match self {
            Const(item) => item.visit(),
            Enum(item) => item.visit(),
            ExternCrate(item) => item.visit(),
            Fn(item) => item.visit(),
            Impl(item) => item.visit(),
            Macro(item) => item.visit(),
            Macro2(item) => item.visit(),
            Mod(item) => item.visit(),
            Static(item) => item.visit(),
            Struct(item) => item.visit(),
            Trait(item) => item.visit(),
            TraitAlias(item) => item.visit(),
            Type(item) => item.visit(),
            Union(item) => item.visit(),
            _ => Default::default(),
        }
    }
}

impl IdentVisitor for syn::Pat {
    fn visit(self) -> Vec<syn::Ident> {
        use syn::Pat::*;
        match self {
            Box(pat) => pat.visit(),
            Ident(ident) => ident.visit(),
            Lit(lit) => lit.visit(),
            Or(or) => or.visit(),
            Range(range) => range.visit(),
            Reference(ref_) => ref_.visit(),
            Slice(slice) => slice.visit(),
            Struct(fields) => fields.visit(),
            Tuple(tuple) => tuple.visit(),
            TupleStruct(tuple) => tuple.visit(),
            Type(ty) => ty.visit(),
            _ => Default::default(),
        }
    }
}

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

impl IdentVisitor for syn::ImplItem {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::ImplItem::Const(item) => item.visit(),
            syn::ImplItem::Method(item) => item.visit(),
            syn::ImplItem::Type(item) => item.visit(),
            syn::ImplItem::Macro(item) => item.visit(),
            _ => Default::default(),
        }
    }
}

impl IdentVisitor for syn::TraitItem {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::TraitItem::Const(item) => item.visit(),
            syn::TraitItem::Method(item) => item.visit(),
            syn::TraitItem::Type(item) => item.visit(),
            syn::TraitItem::Macro(item) => item.visit(),
            _ => Default::default(),
        }
    }
}

impl IdentVisitor for syn::Member {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::Member::Named(ident) => ident.visit(),
            _ => Default::default(),
        }
    }
}

impl IdentVisitor for syn::Fields {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::Fields::Named(fields) => fields.visit(),
            syn::Fields::Unnamed(fields) => fields.visit(),
            syn::Fields::Unit => Default::default(),
        }
    }
}

impl IdentVisitor for syn::FnArg {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::FnArg::Receiver(_) => Default::default(),
            syn::FnArg::Typed(ty) => ty.visit(),
        }
    }
}

impl IdentVisitor for syn::ReturnType {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::ReturnType::Type(_, ty) => ty.visit(),
            _ => Default::default(),
        }
    }
}

impl IdentVisitor for syn::TypeParamBound {
    fn visit(self) -> Vec<syn::Ident> {
        match self {
            syn::TypeParamBound::Trait(trait_) => trait_.visit(),
            _ => Default::default(),
        }
    }
}

impl IdentVisitor for syn::Ident {
    fn visit(self) -> Vec<syn::Ident> {
        vec![self]
    }
}

impl IdentVisitor for syn::Local {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { pat, init, .. } = self;
        pat.visit().chain(init.visit())
    }
}

impl IdentVisitor for syn::BareFnArg {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { name, ty, .. } = self;
        name.map(swap_tuple).visit().chain(ty.visit())
    }
}

impl IdentVisitor for syn::TraitBound {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { path, .. } = self;
        path.visit()
    }
}

impl IdentVisitor for syn::Macro {
    fn visit(self) -> Vec<syn::Ident> {
        self.path.visit()
    }
}

impl IdentVisitor for syn::QSelf {
    fn visit(self) -> Vec<syn::Ident> {
        self.ty.visit()
    }
}

impl IdentVisitor for syn::TypeArray {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, len, .. } = self;
        elem.visit().chain(len.visit())
    }
}

impl IdentVisitor for syn::TypeBareFn {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { inputs, output, .. } = self;
        inputs.visit().chain(output.visit())
    }
}

impl IdentVisitor for syn::TypeImplTrait {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { bounds, .. } = self;
        bounds.visit()
    }
}

impl IdentVisitor for syn::TypeMacro {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { mac, .. } = self;
        mac.visit()
    }
}

impl IdentVisitor for syn::TypeParen {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, .. } = self;
        elem.visit()
    }
}

impl IdentVisitor for syn::TypePath {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { qself, path, .. } = self;
        qself.visit().chain(path.visit())
    }
}

impl IdentVisitor for syn::TypePtr {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, .. } = self;
        elem.visit()
    }
}

impl IdentVisitor for syn::TypeReference {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, .. } = self;
        elem.visit()
    }
}

impl IdentVisitor for syn::TypeGroup {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, .. } = self;
        elem.visit()
    }
}

impl IdentVisitor for syn::TypeSlice {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elem, .. } = self;
        elem.visit()
    }
}

impl IdentVisitor for syn::TypeTraitObject {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { bounds, .. } = self;
        bounds.visit()
    }
}

impl IdentVisitor for syn::TypeTuple {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elems, .. } = self;
        elems.visit()
    }
}

impl IdentVisitor for syn::Path {
    fn visit(self) -> Vec<syn::Ident> {
        self.segments
            .last()
            .cloned()
            .map(|s| s.ident)
            .into_iter()
            .collect()
    }
}

impl IdentVisitor for syn::Field {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, ty, .. } = self;
        ident.visit().chain(ty.visit())
    }
}

impl IdentVisitor for syn::FieldsNamed {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { named, .. } = self;
        named.visit()
    }
}

impl IdentVisitor for syn::FieldsUnnamed {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { unnamed, .. } = self;
        unnamed.visit()
    }
}

impl IdentVisitor for syn::ImplItemMacro {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { mac, .. } = self;
        mac.visit()
    }
}

impl IdentVisitor for syn::ImplItemConst {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident, ty, expr, ..
        } = self;
        ident.visit().chain(ty.visit()).chain(expr.visit())
    }
}

impl IdentVisitor for syn::ImplItemMethod {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { block, .. } = self;
        block.visit()
    }
}

impl IdentVisitor for syn::ImplItemType {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, ty, .. } = self;
        ident.visit().chain(ty.visit())
    }
}

impl IdentVisitor for syn::ItemConst {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, expr, .. } = self;
        ident.visit().chain(expr.visit())
    }
}

impl IdentVisitor for syn::ItemEnum {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident, variants, ..
        } = self;
        ident.visit().chain(variants.visit())
    }
}

impl IdentVisitor for syn::Block {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { stmts, .. } = self;
        stmts.visit()
    }
}

impl IdentVisitor for syn::ItemExternCrate {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, rename, .. } = self;
        ident.visit().chain(rename.visit())
    }
}

impl IdentVisitor for syn::ItemFn {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { sig, block, .. } = self;
        sig.visit().chain(block.visit())
    }
}

impl IdentVisitor for syn::ItemImpl {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            trait_,
            self_ty,
            items,
            ..
        } = self;
        trait_
            .map(|(_, path, _)| path)
            .visit()
            .chain(self_ty.visit())
            .chain(items.visit())
    }
}

impl IdentVisitor for syn::ItemMacro {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, mac, .. } = self;
        ident.visit().chain(mac.visit())
    }
}

impl IdentVisitor for syn::ItemMacro2 {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, .. } = self;
        ident.visit()
    }
}

impl IdentVisitor for syn::ItemMod {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, content, .. } = self;
        ident.visit().chain(content.visit())
    }
}

impl IdentVisitor for syn::ItemStatic {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident, ty, expr, ..
        } = self;
        ident.visit().chain(ty.visit()).chain(expr.visit())
    }
}

impl IdentVisitor for syn::ItemStruct {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, fields, .. } = self;
        ident.visit().chain(fields.visit())
    }
}

impl IdentVisitor for syn::ItemTrait {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident,
            supertraits,
            items,
            ..
        } = self;
        ident
            .visit()
            .chain(supertraits.visit())
            .chain(items.visit())
    }
}

impl IdentVisitor for syn::ItemTraitAlias {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, bounds, .. } = self;
        ident.visit().chain(bounds.visit())
    }
}

impl IdentVisitor for syn::ItemType {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, ty, .. } = self;
        ident.visit().chain(ty.visit())
    }
}

impl IdentVisitor for syn::ItemUnion {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, fields, .. } = self;
        ident.visit().chain(fields.visit())
    }
}

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

impl IdentVisitor for syn::TraitItemMacro {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { mac, .. } = self;
        mac.visit()
    }
}

impl IdentVisitor for syn::TraitItemConst {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident, ty, default, ..
        } = self;
        ident.visit().chain(ty.visit()).chain(default.visit())
    }
}

impl IdentVisitor for syn::TraitItemMethod {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { sig, default, .. } = self;
        sig.visit().chain(default.visit())
    }
}

impl IdentVisitor for syn::TraitItemType {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident,
            bounds,
            default,
            ..
        } = self;
        ident.visit().chain(bounds.visit()).chain(default.visit())
    }
}

impl IdentVisitor for syn::Variant {
    fn visit(self) -> Vec<syn::Ident> {
        let Self {
            ident,
            fields,
            discriminant,
            ..
        } = self;
        ident
            .visit()
            .chain(fields.visit())
            .chain(discriminant.visit())
    }
}

impl IdentVisitor for syn::PatBox {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { pat, .. } = self;
        pat.visit()
    }
}

impl IdentVisitor for syn::PatIdent {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { ident, subpat, .. } = self;
        ident.visit().chain(subpat.visit())
    }
}

impl IdentVisitor for syn::PatLit {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { expr, .. } = self;
        expr.visit()
    }
}

impl IdentVisitor for syn::PatOr {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { cases, .. } = self;
        cases.visit()
    }
}

impl IdentVisitor for syn::PatRange {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { lo, hi, .. } = self;
        lo.visit().chain(hi.visit())
    }
}

impl IdentVisitor for syn::PatReference {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { pat, .. } = self;
        pat.visit()
    }
}

impl IdentVisitor for syn::PatSlice {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elems, .. } = self;
        elems.visit()
    }
}

impl IdentVisitor for syn::PatStruct {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { fields, .. } = self;
        fields.visit()
    }
}

impl IdentVisitor for syn::PatTuple {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { elems, .. } = self;
        elems.visit()
    }
}

impl IdentVisitor for syn::PatTupleStruct {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { pat, .. } = self;
        pat.visit()
    }
}

impl IdentVisitor for syn::PatType {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { pat, .. } = self;
        pat.visit()
    }
}

impl IdentVisitor for syn::FieldPat {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { member, pat, .. } = self;
        member.visit().chain(pat.visit())
    }
}

impl IdentVisitor for syn::ExprAssign {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { left, right, .. } = self;
        left.visit().chain(right.visit())
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

impl IdentVisitor for syn::ExprType {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { expr, .. } = self;
        expr.visit()
    }
}

impl IdentVisitor for syn::ExprField {
    fn visit(self) -> Vec<syn::Ident> {
        let Self { base, .. } = self;
        base.visit()
    }
}

#[derive(Default)]
struct Annotations {
    map: HashMap<String, ()>,
}

impl Annotations {
    // fn insert(&mut self, ast: ()) {
    //     self.map.insert(ast.blob.object.to_string(), ast);
    // }

    fn write_json<W: Write>(&self, _w: W) -> std::io::Result<()> {
        unimplemented!()
    }
}

enum Annotation {
    Link(Link),
    Markdown(Markdown),
}

#[derive(Debug, Clone)]
struct Link {
    lineno: usize,
    colno: usize, // start
    len: usize,   // end
    to: String,   // TODO this shouldn't be a string yet
    title: Option<String>,
    color: Option<String>,
}

impl From<Span> for Link {
    fn from(span: Span) -> Self {
        Self {
            lineno: span.line_start,
            colno: span.column_start,
            len: span.column_end - span.column_start,
            to: "".into(),
            title: None,
            color: None,
        }
    }
}

#[derive(Debug, Clone)]
struct Markdown {
    lineno: usize,
    title: String,
    content: String,
}

#[derive(Debug)]
struct Span {
    line_start: usize,
    line_end: usize,
    column_start: usize,
    column_end: usize,
}

impl From<proc_macro2::Span> for Span {
    fn from(span: proc_macro2::Span) -> Self {
        let proc_macro2::LineColumn {
            line: line_start,
            column: column_start,
        } = span.start();
        let proc_macro2::LineColumn {
            line: line_end,
            column: column_end,
        } = span.end();

        Self {
            line_start,
            column_start,
            line_end,
            column_end,
        }
    }
}

fn swap_tuple<T, E>((left, right): (T, E)) -> (E, T) {
    (right, left)
}

fn main() {
    let mut args = std::env::args();
    let file = match args.nth(1) {
        Some(file) => file,
        None => {
            eprintln!("error: provide a directory");
            std::process::exit(1)
        }
    };

    // TODO I need to find the 'crate' root (e.g. main.rs or lib.rs)
    // probably should see how cargo or rust-analyzer loads workspaces
    // to resolve this

    let base = Path::new(&file);
    if !base.is_dir() {
        println!("> {}", base.display());
        let source = std::fs::read_to_string(base).unwrap();
        scry(&source);
        return;
    }

    let data = git_tree(&file).unwrap();
    use rayon::prelude::*;
    parse_blobs(&data)
        .collect::<Vec<_>>()
        .into_par_iter()
        .for_each(|blob| {
            let name = base.join(blob.path);
            let file = std::fs::read_to_string(&name).expect("file must be readable");

            println!("{}:{}", blob.object, blob.path.display());
            let _ = scry(&file);
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testing() {
        let bad = &[
            "~/dev/forks/rust/src/test/ui/parser/new-unicode-escapes-1.rs",
            "~/dev/forks/rust/src/test/ui/parser/new-unicode-escapes-2.rs",
            "~/dev/forks/rust/src/test/ui/parser/new-unicode-escapes-3.rs",
            "~/dev/forks/rust/src/test/ui/parser/new-unicode-escapes-4.rs",
        ];

        for bad in bad {
            eprintln!("trying: {}", bad);
            let s = std::fs::read_to_string(&bad).unwrap();
            eprintln!("{:#?}", scry(&s));
        }
    }
}
