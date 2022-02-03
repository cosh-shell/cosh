use std::env::{current_exe, temp_dir};
use std::fs::{create_dir, File};
use std::{env, panic};
use std::io::Write;
use std::panic::PanicInfo;
use std::path::Path;
use backtrace::{Backtrace, trace};
use chrono::Utc;
use clearscreen::clear;
use dirs::home_dir;
use guess_host_triple::guess_host_triple;
use rustc_version_runtime::version;
use yansi::Paint;
use crate::err_ln;

pub fn attach_cosh_panic_handler() {
    panic::set_hook(Box::new(|info| handle_panic(info)));
}

fn handle_panic(info: &PanicInfo) {
    #[allow(unused_must_use)]
    clear();
    err_ln("cosh is no longer able to continue execution due to a fault.".to_string());
    println!("{}", Paint::yellow("\nPanic information:").bold());
    if let Some(x) = info.payload().downcast_ref::<&str>() {
        println!("cause            : {:?}", x)
    } else if let Some(x) = info.payload().downcast_ref::<String>() {
        println!("cause            : {:?}", x)
    } else {
        println!("cause            : unknown");
    }
    if let Some(location) = info.location() {
        println!("location         : file '{}' at {}:{}", location.file(), location.line(), location.column());
    } else {
        println!("location         : cannot get location");
    }
    println!("{}", Paint::yellow("\nSystem information:").bold());
    println!("operating system : {}", env::consts::OS);
    println!("system family    : {}", env::consts::FAMILY);
    println!("processor arch   : {}", env::consts::ARCH);
    println!("rustc version    : {}", version());
    println!("target triple    : {}\n", guess_host_triple().unwrap_or("unknown"));
    let backtrace = Backtrace::new();
    let mut temp: u8 = 0;
    for f in backtrace.frames() {
        temp += 1;
        let symbol = f.symbols().get(0).unwrap();
        println!("{} - {:#?} ({})", Paint::cyan(format!("frame {}:", temp)), f.ip(), symbol.name().unwrap());
    }
}