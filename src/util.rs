use std::path::{Path, PathBuf};

pub fn canonicalize(base: impl AsRef<Path>, stem: impl AsRef<Path>) -> PathBuf {
    base.as_ref()
        .join(stem)
        .canonicalize()
        .expect("valid filenames from git")
}

pub fn print_header() {
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

pub fn print_help_and_exit() -> ! {
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
