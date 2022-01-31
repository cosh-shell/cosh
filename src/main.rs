extern crate core;

use std::env::*;
use std::path::Path;
use std::process::Command;
use clearscreen::clear;
use reedline::{Reedline, Signal};
use print::*;
use yansi::Paint;
use crate::builtin::ls;

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
    let mut rl = Reedline::create().unwrap();
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
    loop {
        // print!("{}", prompt);
        // stdout().flush().unwrap();
        let input = rl.read_line(&Cosh::default());
        match input {
            Ok(Signal::Success(res)) => {
                let mut parts = res.trim().split_whitespace();
                let p = parts.next();
                let command = p.unwrap();
                let args = parts;
                match command {
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