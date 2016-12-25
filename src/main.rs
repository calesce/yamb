extern crate glob;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use glob::glob;

fn main() {
    let new_file = get_files_concatenated("js/**/*.js");
    let _ = write_file(new_file.as_str());
}

fn get_files_concatenated(dir: &str) -> String {
    glob(dir).unwrap()
        .filter_map(|file| {
            match file {
                Ok(path) => {
                    println!("{:?}", path.as_path());
                    get_contents_from_path(path.as_path())
                },
                _ => None
            }
        })
        .fold(String::new(), |acc, contents| {
            acc + &contents + "\n"
        })
}

fn get_contents_from_path(path: &Path) -> Option<String> {
    let path_result = read_file(path);
    match path_result {
        Ok(path) => Some(path),
        _ => None
    }
}

fn read_file(path: &Path) -> std::io::Result<(String)> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    Ok((buffer))
}

fn write_file(contents: &str) -> std::io::Result<()> {
    let mut f = File::create("bundle.js")?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}
