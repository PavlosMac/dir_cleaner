// > get cli args, one for *destination *file types(ext.) *operation
#![allow(unused)]

use regex::Regex;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    dir_path: PathBuf,
    extension_str: String,
}
fn main() {
    let args = Cli::parse();
    let path = &args.dir_path;
    let ext = &args.extension_str.to_owned();

    for entry in std::fs::read_dir(&path).unwrap() {
        let entry = entry
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let f = format!("\\.(?i:)(?:{})$", &args.extension_str);
        let r = Regex::new(&f).unwrap();
        if r.find(entry.as_str()).is_some() {
            let p = path.to_string_lossy();
            let f = format!("{}/{}", &p, &entry);
            std::fs::remove_file(f);
        }
    }
}
