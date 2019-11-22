#![allow(dead_code)]
#![feature(proc_macro_span)]

mod args;
mod blob;

mod annotate;

mod visit;
use visit::IdentVisitor;

fn print_header() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

// TODO don't expose this, have Args return an Error that main can use with this
pub fn print_error_and_exit(messages: &[&dyn std::fmt::Display]) -> ! {
    print_header();

    println!("error:");
    for message in messages {
        println!("  {}", message);
    }
    std::process::exit(1)
}

fn print_help_and_exit() -> ! {
    print_header();
    println!("usage:");
    println!("  sourcemap -i bad_files.txt -d my_repo");
    println!("flags:");
    println!("  -i, --ignore <file>");
    println!("    creates an ignore set from the LF separated lines in <file>");
    println!("  -d, --directory <dir>");
    println!("    directory to walk for rust files (using git ls-tree)");
    println!("  -f, --file <file>");
    println!("    parse just this file");
    println!("  -h, --help");
    println!("    print this message");
    std::process::exit(0)
}

fn scry(source: impl AsRef<str>) -> Result<Vec<syn::Ident>, syn::Error> {
    syn::parse_str::<syn::File>(source.as_ref()).map(|file| file.items.visit())
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
            println!("parse dir");
            // do the normal git tree / parse each file
        }
        (None, Some(file)) => {
            if !file.is_dir() {
                print_error_and_exit(&[&format!("'{}' is not a file", file.display())])
            }
            println!("parse file");
            // parse just one file
        }
        (None, None) => print_help_and_exit(),
        _ => print_error_and_exit(&[
            &"--directory and --file were provided",
            &"both cannot be used together",
        ]),
    };
}
