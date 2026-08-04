use std::{error::Error, ffi::OsString};

#[derive(Debug)]
enum ReadDirError {
    FileSystem(std::io::Error),
}

impl Error for ReadDirError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::FileSystem(err) => Some(err),
        }
    }
}

impl std::fmt::Display for ReadDirError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FileSystem(err) => write!(f, "failed to read directory: {err}"),
        }
    }
}

fn read_dir() -> Result<Vec<OsString>, ReadDirError> {
    let entries = std::fs::read_dir(".").map_err(ReadDirError::FileSystem)?;

    entries
        .map(|res| {
            res.map(|entry| entry.file_name())
                .map_err(ReadDirError::FileSystem)
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", read_dir()?);
    return Ok(());
}
