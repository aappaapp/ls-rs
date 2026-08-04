use std::{error::Error, ffi::OsString, fmt::Formatter};

#[derive(Debug)]
enum ReadDirError {
    Environment,
    FileSystem,
}

impl Error for ReadDirError {}

impl std::fmt::Display for ReadDirError {
    fn fmt(&self, _fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

fn read_dir() -> Result<Vec<OsString>, ReadDirError> {
    let entries =
        std::fs::read_dir(std::env::current_dir().map_err(|_| ReadDirError::Environment)?)
            .map_err(|_| ReadDirError::FileSystem)?;

    entries
        .map(|res| {
            res.map(|entry| entry.file_name())
                .map_err(|_| ReadDirError::FileSystem)
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", read_dir()?);
    return Ok(());
}
