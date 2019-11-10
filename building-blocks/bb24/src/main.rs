use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn main() {
    println!("Hello, world!");
    let path = TempDir::new().expect("unable to create temporary working directory");
    open_file(path, "chendan.log".to_string());
}


fn open_file(path : impl Into<PathBuf>, filename: String)  {
    let path = path.into();
    path.join(filename);
}