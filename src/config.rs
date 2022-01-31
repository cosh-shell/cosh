use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::exit;
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use crate::{disable_virtual_terminal_processing, err_ln};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Configuration {
    pub macros: Option<HashMap<String, String>>, // -> macros come in the form <Original-Command, Alias>.
    pub history_capacity: u32 // history capacity maximum lines
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            macros: Default::default(),
            history_capacity: 25
        }
    }
}

#[inline]
pub fn config_dir() -> PathBuf {
    ProjectDirs::from("", "", "cosh").unwrap().config_dir().to_path_buf()
}

/// Attempts to load the config. If not existing, creates a new one and
/// returns [`Configuration::default()`].
pub fn load_config() -> Configuration {
    if !config_dir().join("cosh.toml").exists() {
        match File::create(config_dir().join("cosh.toml")) {
            Ok(mut x) => {
                match x.write_all(toml::to_string_pretty(&Configuration::default()).unwrap().as_bytes()) {
                    Ok(_) => {
                    }
                    Err(_) => {
                        err_ln(format!("cosh: cannot write to cosh.toml in {}", config_dir().to_string_lossy()));
                    }
                }
                return Configuration::default();
            }
            Err(_) => {
                err_ln(format!("cosh: cannot create cosh.toml in {}", config_dir().to_string_lossy()));
            }
        }
    }
    let mut f = File::open(config_dir().join("cosh.toml")).unwrap();
    let res: &mut [u8] = &mut [];
    f.read(res).unwrap();
    let fin = toml::from_slice::<Configuration>(res);
    match fin {
        Ok(x) => {
            return x;
        }
        Err(e) => {
            err_ln("cosh: incorrect configuration - delete the file to reset config".to_string());
            err_ln(format!("cosh: err: {}", e));
            disable_virtual_terminal_processing();
            exit(1);
        }
    }

}