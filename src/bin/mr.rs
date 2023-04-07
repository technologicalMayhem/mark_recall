use std::{
    collections::HashMap,
    env,
    fs::{self, DirBuilder, File},
    io::Read,
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use eyre::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Mark {
        /// Name of the mark to set. If no mark name is provided 'default' will be used instead.
        #[arg(default_value_t = String::from("default"))]
        name: String,
    },
    Recall {
        /// Name of the mark to get. If no mark name is provided 'default' will be used instead.
        #[arg(default_value_t = String::from("default"))]
        name: String,
    },
    Clear {
        /// Name of the mark to delete. If no mark name is provided 'default' will be used instead.
        #[arg(default_value_t = String::from("default"))]
        name: String,
        #[arg(long, default_value_t = false)]
        /// Delete all marks
        all: bool,
    },
    List,
}

#[derive(Debug, Error)]
enum Error {
    #[error("Cannot acess the marks located at '~/.config/marks.list': {0}")]
    CannotAcessMarks(#[from] std::io::Error),
    #[error("Could not read marks located at '~/.config/marks.list'. Is the file malformed?")]
    FailedToDeserialize(#[from] serde_json::Error),
    #[error("No path set for {key}.")]
    NoPathSet { key: String },
    #[error("Could not locate home folder")]
    NoHome,
}

#[derive(Serialize, Deserialize)]
struct Mark {
    name: String,
    path: PathBuf,
}

const MARKS_PATH: &str = ".config/marks.list";

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Mark { name } => mark(&name)?,
        Commands::Recall { name } => println!("{}", recall(&name)?.display()),
        Commands::Clear { name, all } => clear(&name, all)?,
        Commands::List => list()?,
    };

    Ok(())
}

fn marks_path() -> Result<PathBuf, Error> {
    if let Some(mut dir) = home::home_dir() {
        dir.push(PathBuf::from(MARKS_PATH));
        return Ok(dir);
    }
    Err(Error::NoHome)
}

fn load_marks() -> Result<HashMap<String, PathBuf>, Error> {
    let path = marks_path()?;
    if !path.try_exists()? {
        return Ok(HashMap::new());
    };

    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    Ok(serde_json::from_str(&s)?)
}

fn save_marks(marks: &HashMap<String, PathBuf>) -> Result<(), Error> {
    let path = marks_path()?;
    if !path.try_exists()? {
        if let Some(path) = path.parent() {
            DirBuilder::new().recursive(true).create(path)?;
        }
    };

    fs::write(path, serde_json::to_string(&marks)?)?;
    Ok(())
}

fn mark(name: &str) -> Result<(), Error> {
    let mut marks = load_marks()?;
    let path = env::current_dir()?;

    marks.insert(name.to_lowercase(), path);
    save_marks(&marks)?;

    Ok(())
}

fn recall(name: &str) -> Result<PathBuf, Error> {
    let marks = load_marks()?;

    match marks.get(&name.to_lowercase()) {
        Some(path) => Ok(path.clone()),
        None => {
            if name == "_" {
                Err(Error::NoPathSet {
                    key: "the default path".to_string(),
                })
            } else {
                Err(Error::NoPathSet { key: name.to_string() })
            }
        }
    }
}

fn clear(name: &str, all: bool) -> Result<(), Error> {
    let mut marks = load_marks()?;
    if all {
        marks.clear();
    } else {
        marks.remove(&name.to_lowercase());
    }
    save_marks(&marks)?;

    Ok(())
}

fn list() -> Result<(), Error> {
    let marks = load_marks()?;
    for (name, path) in marks {
        println!("{name}: {}", path.display());
    }

    Ok(())
}
