use std::collections::HashMap;
use std::io::Write;

#[derive(Default)]
pub struct Annotations {
    map: HashMap<String, ()>,
}

impl Annotations {
    // fn insert(&mut self, ast: ()) {
    //     self.map.insert(ast.blob.object.to_string(), ast);
    // }

    pub fn write_json<W: Write>(&self, _w: W) -> std::io::Result<()> {
        unimplemented!()
    }
}

pub enum Annotation {
    Link(Link),
    Markdown(Markdown),
}

#[derive(Debug, Clone)]
pub struct Link {
    pub lineno: usize,
    pub colno: usize, // start
    pub len: usize,   // end
    pub to: String,   // TODO this shouldn't be a string yet
    pub title: Option<String>,
    pub color: Option<String>,
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
pub struct Markdown {
    pub lineno: usize,
    pub title: String,
    pub content: String,
}

#[derive(Debug)]
pub struct Span {
    pub line_start: usize,
    pub line_end: usize,
    pub column_start: usize,
    pub column_end: usize,
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
