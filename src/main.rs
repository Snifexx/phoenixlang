#![allow(warnings)]
use core::panic;
use std::path::PathBuf;
use std::fs;

use clap::{Parser, CommandFactory, error::ErrorKind};
use cli::PhoenixCli;
mod cli;

fn main() {
    let cli = PhoenixCli::parse();
    let cmd = PhoenixCli::command();

    match cli.subcmd {
        cli::Commands::Compile { scan, file } => compile_file(scan, file),
        cli::Commands::Run { scan, compiled, debug, file } => todo!(),
    }
}

fn compile_file(scan: bool, file: PathBuf) {
    if let Some(ext) = file.extension() {
        if ext != "phx" { 
            PhoenixCli::command().error(
                ErrorKind::InvalidValue, "File must be either .phx").exit()
        }
    } else { PhoenixCli::command().error(
            ErrorKind::InvalidValue, "File must be either .phx").exit() }

    let src =  fs::read_to_string(file).map_err(|err| panic!("{}", err.to_string())).unwrap();
    
}
