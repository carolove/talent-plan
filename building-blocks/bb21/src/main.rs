use std::path::{Path, PathBuf};
use bb21::Result;
use std::env::current_dir;
use std::fs::{create_dir_all, read_dir};
use std::ffi::OsStr;

fn main() {
    println!("Hello, world!");
    let path = current_dir().unwrap();
    println!("{:?}", sorted_gen_list(path).unwrap());
}

fn sorted_gen_list(path: impl Into<PathBuf>) -> Result<Vec<u64>> {
    let path = path.into();
    create_dir_all(&path)?;

    let mut gen_list : Vec<u64> =
        read_dir(&path)?.flat_map(|res| -> Result<_> {Ok(res?.path())}).filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
            .flat_map(|path| {
                path.file_name()
                    .and_then(OsStr::to_str)
                    .map(|s| s.trim_end_matches(".log"))
                    .map(str::parse::<u64>)
            }).flatten().collect();
    gen_list.sort_unstable();
    Ok(gen_list)
}