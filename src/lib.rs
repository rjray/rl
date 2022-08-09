use std::{
    error::Error,
    fs::{self, Metadata},
    path::PathBuf,
};

type SimpleResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    show_hidden: bool,
}

struct Entry {
    path: PathBuf,
    meta: Metadata,
}

pub fn parse_args() -> SimpleResult<Config> {
    Ok(Config {
        paths: vec![],
        show_hidden: false,
    })
}

pub fn run(config: Config) -> SimpleResult<()> {
    let entries = get_file_entries(&config.paths, config.show_hidden)?;

    for entry in entries {
        println!("{}", entry.path.display());
    }

    Ok(())
}

fn get_file_entries(paths_in: &[String], show_hidden: bool) -> SimpleResult<Vec<Entry>> {
    let mut results = vec![];
    let mut paths = paths_in;
    let mut show_leading_dot_slash = true;
    if paths.len() == 0 {
        paths = &[String::from(".")];
        show_leading_dot_slash = false;
    }

    for name in paths {
        match fs::metadata(name) {
            Err(e) => eprintln!("{}: {}", name, e),
            Ok(meta) => {
                if meta.is_dir() {
                    //
                } else {
                    results.push(Entry {
                        path: PathBuf::from(name),
                        meta,
                    })
                }
            }
        }
    }

    Ok(results)
}
