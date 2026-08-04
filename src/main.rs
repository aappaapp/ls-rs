use std::{error::Error, path::PathBuf};

use clap::Parser;

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

fn read_dir(path: PathBuf) -> Result<Vec<std::fs::DirEntry>, ReadDirError> {
    let entries = std::fs::read_dir(path).map_err(ReadDirError::FileSystem)?;

    entries
        .map(|res| res.map_err(ReadDirError::FileSystem))
        .collect()
}

#[derive(Parser, Debug)]
#[command(version, about = "ls in Rust", long_about = None)]
struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,

    #[arg(short = 'A', long)]
    almost_all: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut dir = read_dir(args.path)?;

    if !args.almost_all {
        dir = dir
            .into_iter()
            .filter(|entry| !entry.file_name().to_string_lossy().starts_with("."))
            .collect();
    }

    dir.sort_by_key(|a| {
        a.file_name()
            .to_string_lossy()
            .trim_start_matches('.')
            .to_string()
    });

    println!(
        "{}",
        dir.iter()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect::<Vec<String>>()
            .join("  ")
    );

    return Ok(());
}
