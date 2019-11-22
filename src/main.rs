#![allow(dead_code)]
#![feature(proc_macro_span)]
use std::collections::HashSet;
use std::path::{Path, PathBuf};

mod args;
mod blob;

mod util;
pub use util::*;

mod annotate;

mod visit;
use visit::IdentVisitor;

fn scry(source: impl AsRef<str>) -> Result<Vec<syn::Ident>, syn::Error> {
    syn::parse_str::<syn::File>(source.as_ref()).map(|file| file.items.visit())
}

fn parse_directory(dir: impl AsRef<Path>, ignore_file: Option<PathBuf>, ignored: HashSet<PathBuf>) {
    let mut bad = vec![];
    for blob in match blob::git_tree(dir.as_ref(), &mut vec![]) {
        Ok(data) => data,
        Err(err) => print_error_and_exit(&[&"cannot get the git reference tree", &err.to_string()]),
    }
    .filter(|s| !ignored.contains(&canonicalize(s.base, s.path)))
    {
        let path = canonicalize(&dir, &blob.path);
        println!("{} -> {}", blob.object, path.display());
        let source = std::fs::read_to_string(&path).expect("file is readable");
        match scry(&source) {
            Ok(data) => {}
            Err(..) => bad.push((blob.object.to_string(), path)),
        }
    }

    if !bad.is_empty() {
        eprintln!("error:");
        eprintln!("  could not parse these files");
        eprintln!("  ensure they are valid syntax that `syn` can handle");
        if let Some(ignore) = ignore_file {
            eprintln!("  if they are not, append them to: {}", ignore.display())
        } else {
            eprintln!(
                "  if they are not, add them to a file with each path separated by a new line"
            );
        }
        eprintln!("  then use -i to exclude these invalid files");

        eprintln!();
        eprintln!("{: <40} : path", "git hash");
        for (object, path) in bad {
            eprintln!("{} : {}", object, path.display())
        }
    }
}

// this doesn't make any sense, but its for testing
fn parse_file(file: impl AsRef<Path>) {
    let source = std::fs::read_to_string(&file).unwrap();
    match scry(&source) {
        Ok(data) => {}
        Err(..) => {
            eprintln!("error:");
            eprintln!("  could not parse this files");
            eprintln!("  ensure it is valid syntax that `syn` can handle");
            eprintln!("  filename: {}", file.as_ref().display())
        }
    }
}

fn main() {
    let args = args::Args::parse_and_validate();

    if args.help {
        print_help_and_exit()
    }

    if let Some(ignored) = args.ignored.as_ref() {
        if ignored.1.is_empty() {
            print_error_and_exit(&[&"ignore file produced no entries"])
        }
    }

    match (args.directory, args.file) {
        (Some(dir), None) => {
            if !dir.is_dir() {
                print_error_and_exit(&[&format!("'{}' is not a directory", dir.display())])
            }
            let (file, set) = match args.ignored {
                Some((file, set)) => (file.into(), set),
                None => (None, Default::default()),
            };
            parse_directory(dir, file, set)
        }
        (None, Some(file)) => {
            if !file.is_dir() {
                print_error_and_exit(&[&format!("'{}' is not a file", file.display())])
            }
            parse_file(file)
        }
        (None, None) => print_help_and_exit(),
        _ => print_error_and_exit(&[
            &"--directory and --file were provided",
            &"both cannot be used together",
        ]),
    };
}
