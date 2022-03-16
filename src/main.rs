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
    extension_str: String,
}
fn main() -> BoxResult<()> {
    let args = Cli::parse();
    let path = &args.dir_path;
    let ext = &args.extension_str.to_owned();
    for entry in std::fs::read_dir(&path)? {
        let entry = entry
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let f = format!("\\.(?i:)(?:{})$", &args.extension_str);
        let reg_x = handle_regex(&f).unwrap();
        println!("{}", entry);
        if reg_x.find(entry.as_str()).is_some() {
            let p = path.to_string_lossy();
            let f = format!("{}/{}", &p, &entry);
            println!("{}", f);
            // std::fs::remove_file(f);
        };
    }
    Ok(())
}

fn handle_regex(p: &str) -> BoxResult<Regex> {
    let reg = Regex::new(&p)?;
    Ok(reg)
}
