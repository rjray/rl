mod opts;

use clap::Parser;
use opts::Cli;
use std::{
    error::Error,
    fs::{self, Metadata},
    path::PathBuf,
};

type SimpleResult<T> = Result<T, Box<dyn Error>>;

struct Entry {
    path: PathBuf,
    meta: Metadata,
}

struct PendingDir {
    path: PathBuf,
    meta: Metadata,
}

pub fn run() -> SimpleResult<()> {
    let config = parse_args();

    Ok(())
}

fn parse_args() -> Cli {
    let cli = Cli::parse();
    println!("all: {:?}", cli.all);
    println!("recursive: {:?}", cli.recursive);
    println!("paths: {:?}", cli.paths);

    cli
}

// fn get_file_entries(paths_in: Vec<String>, config: Config) -> SimpleResult<Vec<Entry>> {
//     let mut results: Vec<Entry> = Vec::new();
//     let mut pending: Vec<PendingDir> = Vec::new();
//     let mut paths: Vec<String> = Vec::new();
//     paths.extend(paths_in);
//     if paths.len() == 0 {
//         paths.push(String::from("."));
//     }
//     let args_length = paths.len();

//     for name in paths {
//         match fs::metadata(&name) {
//             Err(e) => eprintln!("{}: {}", name, e),
//             Ok(meta) => {
//                 if meta.is_dir() {
//                     // If there is just the one directory in our list of paths
//                     // then process it immediately. Otherwise add it to the
//                     // pending list.
//                     if args_length == 1 {
//                         process_directory(&config, &name, &mut results, &mut pending)?;
//                     } else {
//                         pending.push(PendingDir {
//                             path: PathBuf::from(name),
//                             meta,
//                         });
//                     }
//                 } else {
//                     results.push(Entry {
//                         path: PathBuf::from(name),
//                         meta,
//                     })
//                 }
//             }
//         }
//     }

//     Ok(results)
// }

// fn process_directory(
//     config: &Config,
//     name: &String,
//     results: &mut Vec<Entry>,
//     pending: &mut Vec<PendingDir>,
// ) -> SimpleResult<()> {
//     for entry in fs::read_dir(name)? {
//         let entry = entry?;
//         let path = entry.path();
//         let hidden = path.file_name().map_or(false, |file_name| {
//             file_name.to_string_lossy().starts_with('.')
//         });
//         if !hidden || config.show_hidden {
//             let meta = fs::metadata(name)?;
//             if meta.is_dir() && config.recurse {
//                 pending.push(PendingDir {
//                     path: PathBuf::from(path),
//                     meta,
//                 });
//             } else {
//                 results.push(Entry {
//                     path: PathBuf::from(path),
//                     meta,
//                 });
//             }
//         }
//     }
//     Ok(())
// }
