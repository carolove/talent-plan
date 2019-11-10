use std::path::{Path, PathBuf};
use std::io;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::env::current_dir;
use std::io::Write;

fn main() {
    println!("Hello, world!");
    let path = current_dir().unwrap();
    let mut write = new_log_file(&path, 10).unwrap();
    let str = "fdsafdsa";
    write.write(str.as_bytes());
}

fn new_log_file(path : &Path, gen: u64)-> io::Result<File> {
    let pathbuf = log_path(path, gen);
    OpenOptions::new().create(true)
        .write(true)
        .append(true)
        .open(&pathbuf)
}

fn log_path(path : &Path, gen :u64) -> PathBuf {
    path.join(format!("{}.log", gen))
}