// > get cli args, one for *destination *file types(ext.) *operation
#![allow(unused)]

use clap::Parser;
use regex::Regex;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

type BoxResult<T> = Result<T, Box<dyn Error>>;

// mod error;
#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    dir_path: PathBuf,
    file_ext: String,
}
fn main() -> BoxResult<()> {
    let args = Cli::parse();
    let path = &args.dir_path;
    let ext = &args.file_ext.to_owned();
    let mut removed_files: i8 = 0;

    for entry in std::fs::read_dir(&path)? {
        let entry = entry
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let f = format!("\\.(?i:)(?:{})$", &args.file_ext);
        let reg_x = handle_regex(&f).unwrap();
        if reg_x.find(entry.as_str()).is_some() {
            let p = path.to_string_lossy();
            let f = format!("{}/{}", &p, &entry);
            println!("removing {} from os:", f);
            if std::fs::remove_file(f).is_ok() {
                removed_files += 1;
            }
        };
    }
    println!(
        "Removed {} files from {:?} with {} extension.",
        removed_files, &path, &ext
    );
    Ok(())
}

fn handle_regex(p: &str) -> BoxResult<Regex> {
    let reg = Regex::new(&p)?;
    Ok(reg)
}
