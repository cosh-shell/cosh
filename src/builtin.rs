use std::env::{split_paths, var_os};
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::Path;
use is_executable::is_executable;
use yansi::Paint;
use crate::err_ln;

#[cfg(windows)]
fn is_hidden(path: &Path) -> bool {
    use std::os::windows::prelude::*;
    return match path.metadata() {
        Ok(f) => {
            winapi_util::file::is_hidden(f.file_attributes() as u64)
        }
        Err(_) => {
            true
        }
    }
}

#[cfg(unix)]
fn is_hidden(path: &Path) -> bool {
    path.file_name().unwrap().to_string_lossy().starts_with(".")
}

/// Prints a listing of the current files. This is equivalent to Windows' `dir`.
pub fn ls(mut dir: &str, flags: Vec<String>) {
    if dir.is_empty() {
        dir = ".";
    }
    let _long : bool = flags.contains(&"-l".to_string());
    let append : bool = flags.contains(&"-F".to_string());
    let show_hidden: bool = flags.contains(&"-a".to_string());
    for x in flags {
        if x != "-l" && x != "-F" && x != "-a" {
            err_ln(format!("unknown option {}", x));
            return;
        }
    }
    if !Path::new(dir).exists() {
        err_ln("The system cannot find the file specified.".to_string());
        return;
    }
    let mut target = String::new();
    for r in read_dir(dir).unwrap() {
        let res = r.unwrap().path();
        if show_hidden || !is_hidden(res.as_path()) {
            if res.is_dir() {
                target += &*format!("{}", Paint::green(res.file_name().unwrap().to_string_lossy()).bold());
                if append {
                    target.push_str(&*Paint::green('/').bold().to_string());
                }
            } else if res.is_symlink() {
                target += &*format!("{}", Paint::yellow(res.file_name().unwrap().to_string_lossy()).italic());
            } else {
                target += &*format!("{}", Paint::cyan(res.file_name().unwrap().to_string_lossy()));
                if is_executable(res.as_path()) {
                    target.push_str(&*Paint::cyan('*').to_string());
                }
            }
            target += " ";
        }
    }
    println!("{}", target.trim());
}

/// A completer that will autocomplete file paths and executables
/// in the `PATH`.
pub fn autocomplete_targets() -> Vec<String> {
    let mut autocomplete = vec![];
    autocomplete.clear();
    // add built-in commands
    autocomplete.append(
        &mut vec![
            "help"      .to_string(),
            "history"   .to_string(),
            "cls"       .to_string(),
            "pwd"       .to_string(),
            "echo"      .to_string(),
            "exit"      .to_string(),
            "ls"        .to_string(),
            "cd"        .to_string()
        ]
    );

    // PATH var
    let env = var_os("PATH").unwrap();
    let paths = split_paths(&env).collect::<Vec<_>>();
    for p in paths {
        match read_dir(p) {
            Ok(r) => {
                for res in r {
                    let result = res.unwrap().path();
                    let result_path = result.as_path();
                    if !is_hidden(result_path) && is_executable(result_path) {
                        autocomplete.push(
                            result.file_name().unwrap().to_string_lossy()
                                .strip_suffix(&result.extension().unwrap_or(OsStr::new("")).to_string_lossy().to_string())
                                .unwrap_or(&*result.file_name().unwrap().to_string_lossy())
                                .to_string()
                        )
                    }
                }
            }
            Err(_) => {}
        }
    }
    autocomplete.dedup();
    autocomplete
}