use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    GitFailed,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "i/o error: {}", err),
            Error::GitFailed => write!(f, "git failed to rune"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::GitFailed => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Blob<'a> {
    pub object: &'a str,
    pub base: &'a Path,
    pub path: &'a Path,
}

impl<'a> From<&Blob<'a>> for BlobOwned {
    fn from(blob: &Blob<'a>) -> Self {
        Self {
            object: blob.object.to_owned(),
            path: crate::canonicalize(blob.base, blob.path),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlobOwned {
    pub object: String,
    pub path: PathBuf,
}

pub fn git_tree<'a>(
    path: &'a Path,
    buf: &'a mut Vec<u8>,
) -> Result<impl Iterator<Item = Blob<'a>> + 'a, Error> {
    let mut output = std::process::Command::new("git")
        .current_dir(&path)
        .args(&["ls-tree", "-zr", "HEAD"])
        .output()
        .map_err(Error::Io)?;

    if !output.status.success() {
        return Err(Error::GitFailed);
    }

    std::mem::swap(buf, &mut output.stdout);

    let iter = buf
        .split(|&b| b == 0)
        .flat_map(|data| std::str::from_utf8(data))
        .filter_map(|line| line.split(' ').nth(2).map(|l| l.split('\t')))
        .filter_map(move |mut parts| {
            Blob {
                object: parts.next()?,
                base: path,
                path: Path::new(parts.next()?),
            }
            .into()
        })
        .filter(|blob| blob.path.extension().and_then(|s| s.to_str()) == Some("rs"));

    Ok(iter)
}
