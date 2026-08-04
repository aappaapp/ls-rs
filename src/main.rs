use std::error::Error;

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

fn read_dir() -> Result<Vec<std::fs::DirEntry>, ReadDirError> {
    let entries = std::fs::read_dir(".").map_err(ReadDirError::FileSystem)?;

    entries
        .map(|res| res.map_err(ReadDirError::FileSystem))
        .collect()
}

#[derive(Parser, Debug)]
#[command(version, about = "ls in Rust", long_about = None)]
struct Args {
    #[arg(short = 'A', long)]
    almost_all: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut dir = read_dir()?;

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

    let length = dir.len();
    for (i, name) in dir.iter().enumerate() {
        print!("{}", name.file_name().to_string_lossy());
        if i < length - 1 {
            print!("  ");
        } else {
            println!();
        }
    }

    return Ok(());
}
