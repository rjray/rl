mod opts;
mod output;

use clap::Parser;
use opts::Cli;
use std::{
    error::Error,
    fs::{self, Metadata},
    path::{Path, PathBuf},
};

// Simplify cases where an error can be returned, but no specific "normal"
// return value is needed.
type SimpleResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub meta: Metadata,
}

#[derive(Debug)]
pub struct Directory {
    pub path: PathBuf,
    pub name: String,
    pub meta: Metadata,
}

pub fn run() -> SimpleResult<()> {
    // Get the configuration based on the command-line arguments:
    let config = Cli::parse();

    // `paths` is the list of command-line-provided paths to list. It is
    // cloned here so that we aren't borrowing from `config` when we process
    // the list.
    let paths = if config.paths.is_empty() {
        vec![String::from(".")]
    } else {
        config.paths.clone()
    };

    // These will hold the entries and directories for the initial reading of
    // the paths and such passed on the command-line.
    let mut entries: Vec<Entry> = Vec::new();
    let mut dirs: Vec<Directory> = Vec::new();

    if paths.len() == 1 {
        // When there is only one path in the list, it is handled a little
        // differently.
        get_initial_entries(&paths[0], true, &mut entries, &mut dirs, &config)?;
    } else {
        // More than one path specified means that we do at least one level of
        // recursion into (top-level) directories, unless `-d` was passed.
        for path in paths {
            get_initial_entries(
                &path,
                false,
                &mut entries,
                &mut dirs,
                &config,
            )?;
        }
    }

    produce_output(&mut entries, &mut dirs, &config)?;

    Ok(())
}

fn get_initial_entries(
    path: &String,
    solo: bool,
    entries: &mut Vec<Entry>,
    dirs: &mut Vec<Directory>,
    config: &Cli,
) -> SimpleResult<()> {
    match fs::metadata(path) {
        Err(e) => eprintln!("{}: {}", path, e),
        Ok(meta) => {
            if meta.is_dir() {
                // When this is a directory, we may or may not explore its
                // contents, depending on whether it is the only path, etc.
                if solo && !config.recursive {
                    // Process the content of the directory directly and
                    // schedule it for output. If config.recursive was true
                    // then we don't do this, we just queue up the dir for
                    // regular processing.
                    process_directory(path, entries, dirs, config)?;
                } else {
                    // If config.directory is not true, then we add it to
                    // the list of directories so it can be recursively
                    // read. Otherwise, it becomes an entry.
                    if !config.directory {
                        dirs.push(Directory {
                            path: PathBuf::from(path),
                            name: String::from(path),
                            meta,
                        });
                    } else {
                        entries.push(Entry {
                            name: String::from(path),
                            meta,
                        })
                    }
                }
            } else {
                entries.push(Entry {
                    name: String::from(path),
                    meta,
                })
            }
        }
    }

    Ok(())
}

fn process_directory(
    name: &String,
    entries: &mut Vec<Entry>,
    dirs: &mut Vec<Directory>,
    config: &Cli,
) -> SimpleResult<()> {
    if config.all {
        // `fs::read_dir()` does not include the . and .. entries from a
        // directory. This is... unfortunate for actual systems programming.
        let base_path = Path::new(name);
        let dot = base_path.join(".");
        let dot_dot = base_path.join("..");
        entries.push(Entry {
            name: ".".to_string(),
            meta: dot.metadata()?,
        });
        entries.push(Entry {
            name: "..".to_string(),
            meta: dot_dot.metadata()?,
        });
    }
    // Proceed to handle all the entries that can be read from this directory
    for entry in fs::read_dir(name)? {
        let entry = entry?;
        let path = entry.path();
        // This is modified from the chapter 14 code from "Command-Line Rust",
        // and gets the leaf file-name from `path` while also down-converting
        // it to a string.
        let name = path.file_name().map_or(String::from(""), |fname| {
            fname.to_string_lossy().to_string()
        });
        let hidden = name.starts_with('.');
        if !hidden || config.all {
            let meta = entry.metadata()?;
            if meta.is_dir() && config.recursive {
                // Only register this as a directory if -R was passed
                dirs.push(Directory { path, meta, name });
            } else {
                entries.push(Entry { meta, name });
            }
        }
    }
    Ok(())
}

fn produce_output(
    entries: &mut Vec<Entry>,
    dirs: &mut Vec<Directory>,
    config: &Cli,
) -> SimpleResult<()> {
    // Used in tracking whether we need to emit a newline before a given
    // directory name when displaying a sub-dir's content.
    let mut need_newline = !entries.is_empty();

    // Do the entries of this directory.
    entries.sort_by(|a, b| a.name.cmp(&b.name));
    for entry in entries {
        println!("{}", entry.name);
    }

    // Process the sub-dirs of this directory. This will only be done when
    // either a directory is a command-line path or -R is given.
    dirs.sort_by(|a, b| a.name.cmp(&b.name));
    for dir in dirs {
        let mut entries: Vec<Entry> = Vec::new();
        let mut dirs: Vec<Directory> = Vec::new();
        let dir_name = String::from(dir.path.to_string_lossy());

        if need_newline {
            println!();
        } else {
            need_newline |= true;
        }
        println!("{}:", &dir_name);

        process_directory(&dir_name, &mut entries, &mut dirs, config)?;

        produce_output(&mut entries, &mut dirs, config)?;
    }

    Ok(())
}
