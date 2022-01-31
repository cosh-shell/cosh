extern crate core;

use std::env::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use clearscreen::clear;
use reedline::{FileBackedHistory, Reedline, Signal};
use std::string::String;
use print::*;
use yansi::Paint;
use crate::builtin::ls;
use crate::config::{config_dir, load_config};

mod print;
mod builtin;
mod config;
mod permission;

pub const HEADER: &str = r#"
                  _
  ___  ___   ___ | |__
 / __|/ _ \ / __|| '_ \
| (__| (_) |\__ \| | | |
 \___|\___/ |___/|_| |_|
"#;

fn main() {
    enable_virtual_terminal_processing();
    println!(
        "{}cosh 1.0.0 {}\n",
        HEADER,
        Paint::blue(
            format!("[rustc {} on {}]",
                    rustc_version_runtime::version(),
                    guess_host_triple::guess_host_triple().unwrap_or("unknown target triple")
            )
        )
    );
    // /*FIXME_later*/ let config = load_config();
    let coshf_history: PathBuf = config_dir().join(".cosh-history");
    let history_str = coshf_history.to_string_lossy().to_string().replace("\\", "/");
    let history = FileBackedHistory::with_file(25, coshf_history).unwrap();
    let mut rl = Reedline::create().unwrap().with_history(Box::new(history)).unwrap();
    loop {
        let input = rl.read_line(&Cosh::default());
        match input {
            Ok(Signal::Success(res)) => {
                let mut parts = res.trim().split_whitespace();
                let p = parts.next();
                let command : &str;
                match p {
                    None => { continue; }
                    Some(e) => {
                        command = e;
                    }
                }
                let args = parts;
                match command {
                    "pwd" => {
                        println!("{}", current_dir().unwrap().to_string_lossy());
                    }
                    "history" => {
                        println!("Printing history from {}", history_str);
                        rl.print_history().unwrap();
                    }
                    "echo" => {
                        let mut target = String::new();
                        for x in args {
                            target.push_str(&*(x.to_owned() + " "));
                        }
                        println!("{}", target.trim());
                    }
                    "help" => {
                        print_help();
                    }
                    "cosh" => {
                        err_ln("cosh: you are already in cosh, type `help` for help.".to_string());
                    }
                    "cd" => {
                        let new_dir = args.peekable().peek().map_or("/", |x| *x);
                        let root = Path::new(new_dir);
                        if let Err(e) = set_current_dir(&root) {
                            err_ln(format!("cosh: {}", e));
                        }
                    },
                    "clear" => {
                        clear().unwrap();
                    }
                    "ls" => {
                        ls(args.peekable().peek().map_or(".", |x| *x));
                    }
                    "exit" => break,
                    command => {
                        let child = Command::new(command)
                            .args(args)
                            .spawn();
                        match child {
                            Ok(mut child) => { child.wait().unwrap(); },
                            Err(e) => {
                                err_ln(format!("cosh: {}", e));
                            },
                        };
                    }
                }
            }
            Ok(Signal::CtrlC) | Ok(Signal::CtrlD) => {
                rl.print_crlf().unwrap();
            }
            Ok(Signal::CtrlL) => {
                clear().unwrap();
            }
            _ => {}
        }
    }
    disable_virtual_terminal_processing();
}

#[cfg(windows)]
pub fn enable_virtual_terminal_processing() {
    use winapi_util::console::Console;
    if let Ok(mut term) = Console::stdout() {
        let _ = term.set_virtual_terminal_processing(true);
    }
    if let Ok(mut term) = Console::stderr() {
        let _ = term.set_virtual_terminal_processing(true);
    }
}

#[cfg(windows)]
pub fn disable_virtual_terminal_processing() {
    use winapi_util::console::Console;
    if let Ok(mut term) = Console::stdout() {
        let _ = term.set_virtual_terminal_processing(false);
    }
    if let Ok(mut term) = Console::stderr() {
        let _ = term.set_virtual_terminal_processing(false);
    }
}

#[cfg(not(windows))]
pub fn enable_virtual_terminal_processing() {
    // no-op
}

#[cfg(not(windows))]
pub fn disable_virtual_terminal_processing() {
    // no-op
}