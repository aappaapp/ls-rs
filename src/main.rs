use std::error::Error;

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

fn read_dir() -> Result<Vec<std::fs::DirEntry>, ReadDirError> {
    let entries = std::fs::read_dir(".").map_err(ReadDirError::FileSystem)?;

    entries
        .map(|res| res.map_err(ReadDirError::FileSystem))
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{:?}",
        read_dir()?
            .iter()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .filter(|name| !name.starts_with("."))
            .collect::<Vec<String>>()
    );
    return Ok(());
}
