use std::borrow::Cow;
use std::env::current_dir;
use std::path::PathBuf;
use dirs::home_dir;
use reedline::{Prompt, PromptEditMode, PromptHistorySearch, PromptHistorySearchStatus};
use whoami::{hostname, username};
use yansi::Paint;
use crate::permission::is_elevated;

/// Prints in stderr() with ANSI and a newline.
pub fn err_ln(msg: String) {
    eprintln!("{}", Paint::red(msg));
}

/// Prints in stderr() with ANSI.
#[allow(dead_code)]
pub fn err(msg: &str) {
    eprint!("{}", Paint::red(msg));
}

impl Prompt for Cosh {

    fn render_prompt(&self, _screen_width: usize) -> Cow<str> {
        let x = current_dir().unwrap().to_string_lossy().replace("\\", "/").replace(&home_dir().unwrap_or(PathBuf::new()).to_string_lossy().to_string(), "~").to_string();
        Cow::from(format!("{} {}", Paint::yellow(x), Paint::green(username() + "@" + &*hostname())))
    }

    fn render_prompt_indicator(&self, _edit_mode: PromptEditMode) -> Cow<str> {
        if is_elevated() {
            " # ".into()
        } else {
            " $ ".into()
        }
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        Cow::Borrowed("... ")
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: PromptHistorySearch,
    ) -> Cow<str> {
        let prefix = match history_search.status {
            PromptHistorySearchStatus::Passing => "",
            PromptHistorySearchStatus::Failing => "failing ",
        };
        Cow::Owned(format!(
            "({}reverse-search: {})",
            prefix, history_search.term
        ))
    }
}

impl Default for Cosh {
    fn default() -> Self {
        Cosh {}
    }
}

#[derive(Clone)]
pub struct Cosh {}

pub fn print_help() {
    println!(
        "cosh 1.0.0 {}\n{}",
        Paint::blue(
            format!("[rustc {} on {}]",
                    rustc_version_runtime::version(),
                    guess_host_triple::guess_host_triple().unwrap_or("unknown target triple")
            )
        ),
        HELP_STRING
    )
}

const HELP_STRING: &str = r#"
Help ......................................
   <xxx> -> required | [xxx] -> optional
-------------------------------------------
    cd <dir> - changes directory to the
             | given one, and outputs an
             | error message if not valid.

    ls [dir] - lists the given directory,
             | defaults to the current one
             | if not specified.
--------------------------------------------
"#;