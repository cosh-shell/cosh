use std::fs::read_dir;
use std::path::Path;
use yansi::Paint;
use crate::err_ln;

/// Prints a listing of the current files. This is equivalent to Windows' `dir`.
pub fn ls(dir: &str) {
    if !Path::new(dir).exists() {
        err_ln("The system cannot find the file specified.".to_string());
        return;
    }
    let mut target = String::new();
    for r in read_dir(dir).unwrap() {
        let res = r.unwrap().path();
        if res.is_dir() {
            target += &*format!("{} ", Paint::green(res.file_name().unwrap().to_string_lossy()));
        } else if res.is_symlink() {
            target += &*format!("{} ", Paint::yellow(res.file_name().unwrap().to_string_lossy()));
        } else {
            target += &*format!("{} ", Paint::cyan(res.file_name().unwrap().to_string_lossy()));
        }
    }
    println!("{}", target.trim());
}