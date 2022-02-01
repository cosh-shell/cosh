use std::borrow::BorrowMut;
use std::env::*;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use clearscreen::clear;
use reedline::{DefaultCompleter, DefaultHighlighter, DefaultHinter, FileBackedHistory, Reedline, Signal};
use std::string::String;
use nu_ansi_term::Color::DarkGray;
use nu_ansi_term::{Color, Style};
use print::*;
use yansi::Paint;
use crate::builtin::{autocomplete_targets, ls};
use crate::config::config_dir;

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
    let exe_vec = autocomplete_targets();
    let mut hil = DefaultHighlighter::new(exe_vec.clone());
    hil.change_colors(Color::LightPurple, Color::White, Color::White); // highlight special commands
    let mut rl = Reedline::create()
        .unwrap()
        .with_history(Box::new(history))
        .unwrap()
        .with_hinter(Box::new(
            DefaultHinter::default().with_inside_line().with_completer(Box::new(DefaultCompleter::new(exe_vec.clone()))).with_style(Style::new().fg(DarkGray))
        ))
        .with_highlighter(Box::new(hil));

    loop {
        let input = rl.read_line(&Cosh::default());
        match input {
            Ok(Signal::Success(res)) => {
                let may_have_comments = res.trim();
                if may_have_comments.is_empty() {
                    continue;
                }
                may_have_comments.split("#").next().unwrap();
                let mut parts = may_have_comments.split("#").next().unwrap().split_whitespace();
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
                    "autocp" => {
                        println!("cosh: use `autocp-ref` to refresh autocompletion indexes.");
                    }
                    "autocp-ref" => {
                        let mut hil = DefaultHighlighter::new(exe_vec.clone());
                        hil.change_colors(Color::LightPurple, Color::LightGray, Color::LightGray); // highlight special commands
                        rl = Reedline::create()
                            .unwrap()
                            .with_history(Box::new(FileBackedHistory::with_file(25, history_str.parse().unwrap()).unwrap()))
                            .unwrap()
                            .with_hinter(Box::new(
                                DefaultHinter::default().with_inside_line().with_completer(Box::new(DefaultCompleter::new(exe_vec.clone()))).with_style(Style::new().fg(DarkGray))
                            ))
                            .with_highlighter(Box::new(hil));
                        println!("cosh: refreshed indexes");
                    }
                    "pwd" => {
                        println!("{}", current_dir().unwrap().to_string_lossy());
                    }
                    "history" => {
                        if args.peekable().peek().unwrap_or(&"") == &"clear" {
                            match File::create(&history_str) {
                                Ok(_) => {
                                    println!("cosh: history file ({}) emptied", history_str);
                                }
                                Err(e) => {
                                    err_ln(format!("cosh: could not empty ({})", history_str));
                                    err_ln(format!("cosh: error - {}", e));
                                }
                            }
                        } else {
                            println!("cosh: printing history from {}", history_str);
                            rl.print_history().unwrap();
                        }
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
                        err_ln("cosh: if we let you do this, cosh would break :c".to_string());
                    }
                    "cd" => {
                        let new_dir = args.peekable().peek().map_or("/", |x| *x);
                        let root = Path::new(new_dir);
                        if let Err(e) = set_current_dir(&root) {
                            err_ln(format!("cosh: {}", e));
                        }
                    },
                    "cls" => {
                        clear().unwrap();
                    }
                    "ls" => {
                        let mut dir = String::new();
                        let mut proc_args = Vec::<String>::new();
                        for arg in args.peekable().borrow_mut() {
                            if arg.starts_with("-") {
                                if arg.starts_with("--") {
                                    err_ln("cosh: the internal `ls` implementation only recognizes short flags (-l, etc)".to_string());
                                    continue;
                                } else {
                                    proc_args.push(arg.trim().to_owned());
                                }
                            } else {
                                if dir.is_empty() {
                                    dir = arg.trim().to_owned();
                                } else {
                                    err_ln("cosh: `ls` expected one parameter".to_string());
                                    continue;
                                }
                            }
                        }
                        ls(&*dir, proc_args);
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
            Ok(Signal::CtrlC) | Ok(Signal::CtrlD) => {}
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