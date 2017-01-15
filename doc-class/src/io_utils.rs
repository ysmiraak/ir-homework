use std::fs::{read_dir, ReadDir, File};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::io;

/// useful in main.
pub fn open_file<P>(path: P) -> File where P: AsRef<Path>{
    match File::open(&path) {
        Err(_) => {
            println!("cannot open file: {:?}", path.as_ref());
            exit(1)
        }
        Ok(file) => file,
    }
}

/// useful in main.
pub fn create_file<P>(path: P) -> File where P: AsRef<Path> {
    match File::create(&path) {
        Err(_) => {
            println!("cannot create file: {:?}", path.as_ref());
            exit(1)
        }
        Ok(file) => file,
    }
}

/// useful in main.
pub fn iter_file_paths<P>(dir: P) -> FilePaths where P: AsRef<Path> {
    match file_paths(dir) {
        Err(err) => {
            println!("cannot open directory: {}", err);
            exit(1)
        }
        Ok(file_paths) => file_paths
    }
}

/// returns an iterator which recursively list all `PathBuf` of files under `dir`.
pub fn file_paths<P>(dir: P) -> io::Result<FilePaths> where P: AsRef<Path> {
    let mut stack = Vec::new();
    stack.push(try!(read_dir(dir)));
    Ok(FilePaths(stack))
}

/// see `file_paths`.
#[derive(Debug)]
pub struct FilePaths(Vec<ReadDir>);

impl Iterator for FilePaths {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        match self.0.pop() {
            None => None,
            Some(mut dir) => {
                match dir.next() {
                    None => self.next(),
                    Some(res_entry) => {
                        self.0.push(dir);
                        match res_entry {
                            Err(_) => self.next(),
                            Ok(entry) => {
                                let path = entry.path();
                                if path.is_file() {
                                    Some(path)
                                } else {
                                    match read_dir(path) {
                                        Err(_) => self.next(),
                                        Ok(dir) => {
                                            self.0.push(dir);
                                            self.next()}}}}}}}}}
    }
}
