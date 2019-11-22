use crate::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Args {
    pub help: bool,
    pub ignored: Option<(PathBuf, HashSet<PathBuf>)>,
    pub directory: Option<PathBuf>,
    pub file: Option<PathBuf>,
}

impl Args {
    // TODO don't exit here, return results
    pub fn parse_and_validate() -> Self {
        use pico_args::Error as P;
        use Error as E;

        match Args::parse(pico_args::Arguments::from_env()) {
            Ok(args) => args,
            Err(E::ParsingArgs(P::UnusedArgsLeft(args))) => {
                print_error_and_exit(&[&format!("unknown arguments: {}", args.join(", "))])
            }
            Err(E::ParsingArgs(P::OptionWithoutAValue(opt))) => {
                print_error_and_exit(&[&format!("option requires a value: {}", &opt)])
            }
            Err(Error::InvalidIgnoreEntry(path, entry)) => print_error_and_exit(&[
                &format!("invalid ignore entry in '{}'", path.display()),
                &format!("tried to read: '{}'", entry.display()),
            ]),
            Err(Error::InvalidIgnoreFile(path, err)) => print_error_and_exit(&[
                &"cannot find ignore file",
                &format!("  {}", err),
                &format!("  tried to read: '{}'", path.display()),
            ]),
            _ => unreachable!("no other errors should be produced"),
        }
    }

    fn parse(mut args: pico_args::Arguments) -> Result<Self, Error> {
        let ignore_file = args
            .opt_value_from_str(["-i", "--ignored"])
            .map_err(Error::ParsingArgs)?;

        let this = Self {
            help: args.contains(["-h", "--help"]),

            ignored: {
                match ignore_file {
                    Some(file) => {
                        let set = build_ignore_set(&file)?;
                        (file, set).into()
                    }
                    None => None,
                }
            },

            directory: args
                .opt_value_from_str(["-d", "--directory"])
                .map_err(Error::ParsingArgs)?,

            file: args
                .opt_value_from_str(["-f", "--fiile"])
                .map_err(Error::ParsingArgs)?,
        };
        args.finish().map(|_| this).map_err(Error::ParsingArgs)
    }
}

#[derive(Debug)]
enum Error {
    ParsingArgs(pico_args::Error),
    InvalidIgnoreEntry(PathBuf, PathBuf),
    InvalidIgnoreFile(PathBuf, std::io::Error),
}

fn build_ignore_set(path: impl AsRef<Path>) -> Result<HashSet<PathBuf>, Error> {
    std::fs::read_to_string(&path)
        .map_err(|err| Error::InvalidIgnoreFile(path.as_ref().to_owned(), err))
        .and_then(|file| {
            file.lines()
                .map(PathBuf::from)
                .map(|s| {
                    if s.is_file() {
                        Ok(s)
                    } else {
                        Err(Error::InvalidIgnoreEntry(path.as_ref().to_owned(), s))
                    }
                })
                .collect()
        })
}
