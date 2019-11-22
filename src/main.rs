#![feature(proc_macro_span)]
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

mod blob;

mod visit;
use visit::IdentVisitor;

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

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}..{}",
            self.line_start,
            self.column_start,
            self.column_end - self.column_start
        )
    }
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

fn scry(source: impl AsRef<str>) -> Result<Vec<syn::Ident>, syn::Error> {
    syn::parse_str::<syn::File>(source.as_ref()).map(|file| file.items.visit())
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

        for ident in scry(&source).into_iter().flatten() {
            println!("{} -> {}", ident, Span::from(ident.span()))
        }
        return;
    }

    let data = blob::git_tree(&file).unwrap();
    for blob in blob::parse_blobs(&data) {
        let name = base.join(blob.path);
        let file = std::fs::read_to_string(&name).expect("file must be readable");

        println!("{} @ {}", blob.object, blob.path.display());
        match scry(&file) {
            Ok(items) => println!("idents: {}", items.len()),
            Err(err) => println!("could not parse: {}", err),
        };
        println!()
    }
}
